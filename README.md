# Lightning Wire Messages
A crate defining serialization and deserialization for lightning wire messages into rust structs.

## Usage
This crate defines the following traits:

### WireItem
Implemented for any type that can be included in a wire message. Requires `encode` and `decode`. Provides methods for TLV serialization and deserialization.

### WireMessage
Implemented for any struct that is a lightning wire message type. Has methods for serialization and deserialization. If `check_type` is false for `read_from`, it is expecting the message without the first 2 bytes indicating the message type. If it is true, it will read in the first 2 bytes and verify it matches the message type, otherwise it will return `std::io::ErrorKind::InvalidData`.

### AnyWireMessage
Can be derived for any enum that is an arbitrary subset of all types that implement `WireMessage`. It will use the first 2 bytes of the message to determine which variant to deserialize into.

## Contributing
Most lightning messages can be trivially implemented using the following derive macros:

### WireMessage

#### Requirements
 - Requires attribute: `#[msg_type = 123]` which defines the 2 byte number used to uniquely identify the message type.
 - The type of each field must implement `WireItem`.
 - Fields can be tagged with the `#[tlv_type = 123]` attribute.
 - TLV field numbers must be monotonically increasing.
 - TLV fields must be after non-TLV fields.
 - TLV fields must be `Option`al.

#### Result
Given a struct named MessageName:
  - Defines an enum called `MessageNameItem` which is a tagged union between all WireItem types within the message.
  - Defines a type called `MessageNameIter` which is an iterator over the messages fields wrapped in `MessageNameItem`.
  - Implements `IntoIterator` for `&MessageName` which converts it to `MessageNameIter`.
  - Implements `WireMessage` for `MessageName`

### AnyWireMessage

#### Requirements
 - Must be an enum.
 - Each variant must contain a single unnamed field that implements `WireMessage`.

#### Result
 - Implements `AnyWireMessage` for the enum.