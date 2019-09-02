# Lightning Wire Messages
A crate defining serialization and deserialization for lightning wire messages into rust structs.

## Usage
This crate defines the following traits:

### WireItem
- Implemented for all wire types.
- Convenience pairing for `WireItemWriter` and `WireItemReader`

#### WireItemWriter
- Requires `encode`.
- Allows an item to be written to the wire.

#### WireItemReader
- Requires `decode`
- Allows an item to be read from the wire.

### TLVWireItem
- Convenience pairing for `TLVWireItemWriter` and `TLVWireItemReader`
- Blanket implementation for WireItem

#### TLVWireItemWriter
- Requires `encode`.
- Provides `encode_tlv`.

#### TLVWireItemReader
- Requires `decode`.
- Provides `decode_tlv`.

### WireMessage
- Implemented for any struct that is a lightning wire message type.
- Convenience pairing for `WireMessageWriter` and `WireMessageReader`.
- Has methods for serialization and deserialization. 
- If `check_type` is false for `decode`, it is expecting the message without the first 2 bytes indicating the message type.
- If it is true, it will read in the first 2 bytes and verify it matches the message type, otherwise it will return `std::io::ErrorKind::InvalidData`.

#### WireMessageWriter
- Requires `encode`.

#### WireMessageReader
- Requires `decode`.

### AnyWireMessage
- Can be derived for any enum that is an arbitrary subset of all types that implement `WireMessage`. 
- It will use the first 2 bytes of the message to determine which variant to deserialize into.

#### AnyWireMessageWriter
- Can be derived for any enum that is an arbitrary subset of all types that implement `WireMessageWriter`. 

#### AnyWireMessageReader
- Can be derived for any enum that is an arbitrary subset of all types that implement `WireMessageReader`. 

## Contributing
Most lightning messages can be trivially implemented using the following derive macros:

### WireMessage
- Optionally can derive only `WireMessageWriter` or `WireMessageReader`.

#### Requirements
 - Requires attribute: `#[msg_type = 123]` which defines the 2 byte number used to uniquely identify the message type.
 - The type of each non-tlv field must implement `WireItemWriter + WireItemReader`.
    - For `WireMessageWriter`, only requires `WireItemWriter`.
    - For `WireMessageReader`, only requires `WireItemReader`.
 - Fields can be tagged with the `#[tlv_type = 123]` attribute.
 - TLV field numbers must be monotonically increasing.
 - TLV fields must be after non-TLV fields.
 - TLV fields must be an `Option<T> where T: TLVWireItemWriter + TLVWireItemReader`.
    - For `WireMessageWriter`, only requires `TLVWireItemWriter`.
    - For `WireMessageReader`, only requires `TLVWireItemReader`.

### AnyWireMessage
- Optionally can derive only `AnyWireMessageWriter` or `AnyWireMessageReader`

#### Requirements
 - Must be an enum.
 - Each variant must contain a single unnamed field that implements `WireMessageWriter + WireMessageReader`.
   - For `AnyWireMessageWriter`, only requires `WireMessageWriter`.
   - For `AnyWireMessageReader`, only requires `WireMessageReader`.

## Benchmark
Tested 1,000,000 serializations and deserializations of the `watchtower::Init` message, for both this crate and lnd with the following results:
 - lightning-wire-msgs: `387.640625ms`
 - lnd: `1.349666231s`

Code for lnd benchmark can be found at `bench/bench.go`.

Code for crate benchmark can be found at the end of `src/lib.rs`.

I urge anyone else to verify these benchmarks, however I am fairly confident at this point that my crate cuts serialization + deserialization time down by about 70%. 