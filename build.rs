#[cfg(feature = "expand")]
mod expand {
    use {
        lightning_wire_msgs_derive_base::{
            any_wire_message_derive, any_wire_message_reader_derive,
            any_wire_message_writer_derive, try_from_derive, wire_message_derive,
            wire_message_reader_derive, wire_message_writer_derive,
        },
        proc_macro2::TokenStream,
        std::env,
        std::fs::File,
        std::io::{Read, Write},
        std::path::Path,
    };

    #[derive(Debug)]
    enum Derives {
        TryFromPrimitive,
        WireMessage,
        WireMessageReader,
        WireMessageWriter,
        AnyWireMessage,
        AnyWireMessageReader,
        AnyWireMessageWriter,
    }

    struct Expander(syn::File);
    impl Expander {
        pub fn new<R: Read>(r: &mut R) -> Self {
            use std::str::FromStr;
            use syn::parse::Parse;
            let mut s = String::new();
            r.read_to_string(&mut s);
            Expander(
                syn::parse2(TokenStream::from_str(&s).expect("parse error")).expect("parse error"),
            )
        }
        pub fn write<W: Write>(mut self, w: &mut W) -> std::io::Result<()> {
            use quote::ToTokens;
            let items = self.0.items;
            self.0.items = Vec::with_capacity(items.len());
            for item in items {
                // match &tree {
                //     TokenTree::Group(g) => {
                //         // let trees: Vec<_> = g.stream().into_iter().collect();
                //     }
                //     _ => (),
                // }
                self.0.items.push(item);
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
