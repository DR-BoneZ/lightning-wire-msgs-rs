#[cfg(feature = "expand")]
mod expand {
    use {
        lightning_wire_msgs_derive_base::{any_wire_msg, try_from, wire_msg},
        proc_macro2::TokenStream,
        std::env,
        std::fs::File,
        std::io::{Read, Write},
        std::path::Path,
        syn::visit_mut::VisitMut,
    };

    enum Derives {
        TryFromPrimitive,
        WireMessage,
        WireMessageReader,
        WireMessageWriter,
        AnyWireMessage,
        AnyWireMessageReader,
        AnyWireMessageWriter,
    }
    impl Derives {
        pub fn as_str(&self) -> &'static str {
            use Derives::*;
            match self {
                TryFromPrimitive => "TryFromPrimitive",
                WireMessage => "WireMessage",
                WireMessageReader => "WireMessageReader",
                WireMessageWriter => "WireMessageWriter",
                AnyWireMessage => "AnyWireMessage",
                AnyWireMessageReader => "AnyWireMessageReader",
                AnyWireMessageWriter => "AnyWireMessageWriter",
            }
        }
        pub fn is_any(path: &syn::Path) -> bool {
            use Derives::*;
            match path {
                p if p.is_ident(TryFromPrimitive.as_str()) => true,
                p if p.is_ident(WireMessage.as_str()) => true,
                p if p.is_ident(WireMessageWriter.as_str()) => true,
                p if p.is_ident(WireMessageReader.as_str()) => true,
                p if p.is_ident(AnyWireMessage.as_str()) => true,
                p if p.is_ident(AnyWireMessageWriter.as_str()) => true,
                p if p.is_ident(AnyWireMessageReader.as_str()) => true,
                _ => false,
            }
        }
    }

    fn strip_derives(attrs: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        attrs
            .into_iter()
            .filter_map(|mut a| match &a.parse_meta() {
                Ok(syn::Meta::NameValue(b)) if b.path.is_ident("msg_type") => None,
                Ok(syn::Meta::List(b)) if b.path.is_ident("derive") => {
                    let nested: syn::punctuated::Punctuated<_, syn::token::Comma> = b
                        .nested
                        .iter()
                        .filter(|m| match m {
                            syn::NestedMeta::Meta(m) => !Derives::is_any(m.path()),
                            _ => true,
                        })
                        .cloned()
                        .collect();
                    if nested.is_empty() {
                        return None;
                    }
                    a.tokens = quote::quote!((#nested));
                    Some(a)
                }
                _ => Some(a),
            })
            .collect()
    }

    fn add_auto_derive_doc(i: &mut syn::Item) {
        match i {
            syn::Item::Impl(i) => i.attrs.push(syn::Attribute {
                pound_token: syn::Token![#](proc_macro2::Span::call_site()),
                style: syn::AttrStyle::Outer,
                bracket_token: syn::token::Bracket(proc_macro2::Span::call_site()),
                path: syn::Path::from(syn::Ident::new("doc", proc_macro2::Span::call_site())),
                tokens: quote::quote!(= "automatically generated"),
            }),
            _ => (),
        };
    }

    struct StripTLVTypes;
    impl VisitMut for StripTLVTypes {
        fn visit_field_mut(&mut self, i: &mut syn::Field) {
            i.attrs.retain(|a| !a.path.is_ident("tlv_type"));
        }
    }

    struct Expander(syn::File);
    impl Expander {
        pub fn new<R: Read>(r: &mut R) -> Self {
            use std::str::FromStr;
            let mut s = String::new();
            r.read_to_string(&mut s).expect("read");
            Expander(
                syn::parse2(TokenStream::from_str(&s).expect("parse error")).expect("parse error"),
            )
        }
        pub fn write<W: Write>(mut self, w: &mut W) -> std::io::Result<()> {
            use quote::ToTokens;
            let items = self.0.items;
            self.0.items = Vec::with_capacity(items.len());
            for mut item in items {
                match &mut item {
                    syn::Item::ExternCrate(a) if a.ident == "lightning_wire_msgs_derive" => (),
                    syn::Item::Struct(ref mut a) => {
                        StripTLVTypes.visit_item_struct_mut(a);
                        let mut new_item = a.clone();
                        new_item.attrs = strip_derives(new_item.attrs);
                        self.0.items.push(syn::Item::Struct(new_item));
                        for func in a
                            .attrs
                            .iter()
                            .filter_map(|a| a.parse_meta().ok())
                            .filter_map(|a| match a {
                                syn::Meta::List(a) => Some(a),
                                _ => None,
                            })
                            .filter(|a| a.path.is_ident("derive"))
                            .flat_map(|a| a.nested.iter().cloned().collect::<Vec<_>>())
                            .filter_map(|a| match a {
                                syn::NestedMeta::Meta(syn::Meta::Path(a)) => Some(a),
                                _ => None,
                            })
                            .filter_map(|a| match &a {
                                a if a.is_ident(Derives::WireMessage.as_str()) => Some(
                                    wire_msg::impl_trait
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                a if a.is_ident(Derives::WireMessageWriter.as_str()) => Some(
                                    wire_msg::impl_writer
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                a if a.is_ident(Derives::WireMessageReader.as_str()) => Some(
                                    wire_msg::impl_reader
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                _ => None,
                            })
                            .next()
                        {
                            let mut i = syn::parse2(func(
                                &syn::parse2(item.to_token_stream())
                                    .expect("convert to derive input"),
                            ))
                            .expect("parse macro output");
                            add_auto_derive_doc(&mut i);
                            self.0.items.push(i);
                        }
                    }
                    syn::Item::Enum(a) => {
                        let mut new_item = a.clone();
                        new_item.attrs = strip_derives(new_item.attrs);
                        self.0.items.push(syn::Item::Enum(new_item));
                        match a
                            .attrs
                            .iter()
                            .filter_map(|a| a.parse_meta().ok())
                            .filter_map(|a| match a {
                                syn::Meta::List(a) => Some(a),
                                _ => None,
                            })
                            .filter(|a| a.path.is_ident("derive"))
                            .flat_map(|a| a.nested.iter().cloned().collect::<Vec<_>>())
                            .filter_map(|a| match a {
                                syn::NestedMeta::Meta(syn::Meta::Path(a)) => Some(a),
                                _ => None,
                            })
                            .filter_map(|a| match &a {
                                a if a.is_ident(Derives::TryFromPrimitive.as_str()) => Some(
                                    try_from::impl_trait
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                a if a.is_ident(Derives::AnyWireMessage.as_str()) => Some(
                                    any_wire_msg::impl_trait
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                a if a.is_ident(Derives::AnyWireMessageWriter.as_str()) => Some(
                                    any_wire_msg::impl_writer
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                a if a.is_ident(Derives::AnyWireMessageReader.as_str()) => Some(
                                    any_wire_msg::impl_reader
                                        as fn(ast: &syn::DeriveInput) -> TokenStream,
                                ),
                                _ => None,
                            })
                            .next()
                        {
                            Some(func) => {
                                let mut i = syn::parse2(func(
                                    &syn::parse2(item.to_token_stream())
                                        .expect("convert to derive input"),
                                ))
                                .expect("parse macro output");
                                add_auto_derive_doc(&mut i);
                                self.0.items.push(i);
                            }
                            _ => (),
                        };
                    }
                    _ => self.0.items.push(item),
                }
            }
            w.write(format!("{}", self.0.to_token_stream()).as_bytes())?;
            Ok(())
        }
    }
    fn process_file<I: AsRef<Path>, O: AsRef<Path>>(in_path: I, out_path: O) {
        let mut i = File::open(in_path.as_ref()).expect("open reader");
        let e = Expander::new(&mut i);
        let mut o = File::create(out_path.as_ref())
            .or_else(|_| File::open(out_path.as_ref()))
            .expect("open writer");
        e.write(&mut o).expect("write");
        let _format = {
            use rustfmt_nightly::*;
            Session::<'_, File>::new(
                Config::default(),
                Some(&mut File::open(out_path.as_ref()).expect("open output for format")),
            )
            .format(Input::File(out_path.as_ref().to_path_buf()))
            .expect("format");
        };
    }

    fn process_dir<I: AsRef<Path>, O: AsRef<Path>, R: AsRef<Path>>(
        in_base: I,
        out_base: O,
        rel: R,
    ) {
        std::fs::create_dir(out_base.as_ref().join(rel.as_ref()))
            .or_else(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Err(e)
                }
            })
            .expect("create src subdir");
        for entry in std::fs::read_dir(in_base.as_ref().join(rel)).expect("read dir") {
            let entry = entry.expect("read entry");
            let epath = entry.path();
            let new_rel = epath.strip_prefix(&in_base).expect("strip prefix");
            if entry.metadata().expect("get metadata").is_dir() {
                process_dir(in_base.as_ref(), out_base.as_ref(), new_rel)
            } else {
                process_file(entry.path(), out_base.as_ref().join(new_rel))
            }
        }
    }
    pub fn main() {
        let out_dir = env::var("OUTPUT_DIR");
        let cwd = env::current_dir().expect("get current dir");
        let in_path = cwd.join("src");
        let mut out_path = out_dir
            .as_ref()
            .map(Path::new)
            .map(Path::to_path_buf)
            .unwrap_or_else(|_| cwd.clone())
            .join("expanded");
        std::fs::create_dir(&out_path)
            .or_else(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Err(e)
                }
            })
            .expect("create output dir");

        std::fs::copy(cwd.join("Cargo_expanded.toml"), out_path.join("Cargo.toml"))
            .expect("copy Cargo.toml");
        std::fs::copy(cwd.join(".gitignore"), out_path.join(".gitignore"))
            .expect("copy .gitignore");

        out_path.push("src");
        process_dir(in_path, out_path, "");
    }
}

fn main() {
    #[cfg(feature = "expand")]
    expand::main();
}
