# shapely

[![experimental](https://img.shields.io/badge/status-highly%20experimental-orange)](https://github.com/fasterthanlime/shapely)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)
[![crates.io](https://img.shields.io/crates/v/shapely.svg)](https://crates.io/crates/shapely)
[![documentation](https://docs.rs/shapely/badge.svg)](https://docs.rs/shapely)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/shapely.svg)](./LICENSE)

> [!IMPORTANT]
>
> There is no stable shapely API as of now (even though it's >1.0.0). The design
> is very much still being explored.
>
> Expect multiple major versions in the near future — (note left 2025-03-11)

shapely provides runtime reflection for Rust.

Any type that implements `Shapely` trait returns a `Shape`, which describes:

  * The memory layout of the type
  * Its innards: struct fields, underlying type for newtypes, etc.
  * How to drop it in place

The `Partial` type is able to allocate (or work from a `&mut MaybeUninit<T>`)
any Shapely type, and gradually initialize its fields — until the fully-built
value is moved out of the partial.

It comes with a derive macro that uses [unsynn](https://crates.io/crates/unsynn)
for speed of compilation.

## Supported Formats

shapely supports deserialization from multiple data formats through dedicated crates:

- [shapely-json](../shapely-json): JSON deserialization
- [shapely-yaml](../shapely-yaml): YAML deserialization
- [shapely-msgpack](../shapely-msgpack): MessagePack deserialization
- [shapely-urlencoded](../shapely-urlencoded): URL-encoded form data deserialization

## Implementing Your Own Deserializer

To implement a custom deserializer for a new format, you'll need to work with the following key components from shapely:

### Key Types

- `Partial`: The central type for building shapely values incrementally
- `Shape`: Describes the memory layout and structure of a type
- `Innards`: Represents the internal structure (Scalar, Struct, etc.)
- `Scalar`: Represents primitive types like String, u64, etc.

### Implementation Pattern

1. Create a function that takes a `&mut Partial` and your format's input (string, bytes, etc.)
2. Examine the shape of the partial using `partial.shape()`
3. Handle different shapes based on `shape.innards`:
   - For `Innards::Scalar`, use `partial.scalar_slot()` to get and fill the slot
   - For `Innards::Struct`, iterate through fields, using `partial.slot_by_name(field_name)` to access each field
   - Create nested `Partial` instances for complex fields and fill them recursively

### Example Implementation Skeleton

```rust
pub fn from_my_format(partial: &mut Partial, input: &[u8]) -> Result<(), MyFormatError> {
    let shape_desc = partial.shape();
    let shape = shape_desc.get();
    
    match &shape.innards {
        Innards::Scalar(scalar) => {
            let slot = partial.scalar_slot().expect("Scalar slot");
            // Parse scalar value from input and fill slot
            match scalar {
                Scalar::String => slot.fill(/* parsed string */),
                Scalar::U64 => slot.fill(/* parsed u64 */),
                // Handle other scalar types
                _ => return Err(MyFormatError::UnsupportedType),
            }
        },
        Innards::Struct { .. } => {
            // Parse struct fields from input
            for (field_name, field_value) in /* parsed fields */ {
                let slot = partial.slot_by_name(field_name)?;
                
                // Create a partial for the field and fill it recursively
                let mut partial_field = Partial::alloc(slot.shape());
                // Recursively deserialize field_value into partial_field
                // ...
                slot.fill_from_partial(partial_field);
            }
        },
        // Handle other shapes as needed
        _ => return Err(MyFormatError::UnsupportedShape),
    }
    
    Ok(())
}
```

For more detailed examples, examine the implementation of existing deserializers like [shapely-json](../shapely-json/src/lib.rs) or [shapely-msgpack](../shapely-msgpack/src/lib.rs).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
