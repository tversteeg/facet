## Hacking Guide to Shapely

## The Shapely Trait and Its Purpose

The `Shapely` trait is the cornerstone of our reflection system. It provides a way to access type information at both compile time and runtime, enabling powerful meta-programming capabilities while maintaining Rust's safety guarantees.

```rust
pub trait Shapely: 'static {
    /// A dummy value of this type, used for compile-time type information
    const DUMMY: Self;

    /// A static reference to a Shape describing this type
    const SHAPE: &'static Shape;
}
```

### Core Concept

The `Shapely` trait allows any implementing type to expose its structural information through a static `Shape` object. This enables introspection of types at compile time, powering serialization, deserialization, debugging, and other operations that need to understand the structure of data.

## Specialization via Auto-Deref

Shapely uses a technique called "auto-deref-based specialization" to enable trait-like specialization on stable Rust. This approach allows us to conditionally implement functionality based on what traits a type implements, all without requiring the unstable `specialization` feature.

### The DUMMY Value

The `DUMMY` constant in the `Shapely` trait might seem strange at first glance. It's important to understand that:

1. The `DUMMY` value is never actually read at runtime
2. It only exists to enable the specialization pattern
3. It just needs to not trigger undefined behavior if it were to be constructed

This is used in conjunction with our specialization system in the `spez.rs` module.

### How Auto-Deref Specialization Works

The specialization technique uses Rust's method resolution rules to our advantage. As described by Lukas Kalbertodt:

> Autoderef-based specialization works by (ab)using the fact that method resolution prefers resolving to methods which require fewer type coercion of the receiver over methods that require more coercions.

For example, in our code:

```rust
// For types that implement Debug
impl<T: Debug> SpezDebugYes for &Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Debug::fmt(&self.0, f)
    }
}

// Fallback for types that don't implement Debug
impl<T> SpezDebugNo for Spez<T> {
    fn spez_debug(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}
```

When we use these traits, the compiler will prefer the first implementation if the type implements `Debug`, and fall back to the second one otherwise. This enables a form of specialization without requiring the unstable feature.

### Limitations of Auto-Deref Specialization

It's important to understand when this specialization technique can and cannot be used:

1. **Only works in macros and non-generic contexts**: Auto-deref specialization is primarily useful in macros (like those in `shapely-derive`) and for non-generic, scalar types like `i32`, `u32`, `String`, etc.

2. **Not suitable for generic types**: For types with generic parameters (like `HashMap<K, V>`), this approach doesn't work well because the specialization cannot be done based on properties of the generic parameters.

3. **Alternative for generic types**: For generic types, we instead leverage the fact that `SHAPE` is a `const` associated value that can be queried in `const` contexts. This allows us to perform compile-time checks and conditional logic based on properties of the generic parameters.

For example, in a generic implementation like `HashMap<K, V>`, we directly access the marker traits of `K::SHAPE` and `V::SHAPE` at compile time to determine what traits to implement for the containing type.

This pattern is used throughout the codebase for various traits like `Debug`, `Display`, `Clone`, `Hash`, and more, with different specialization approaches depending on whether we're dealing with non-generic or generic types.

## Concrete Example: PartialOrd Specialization

Let's examine how the `PartialOrd` trait is conditionally implemented using both approaches:

### Generic Type Example: Array Implementation

For arrays like `[T; 1]`, we need to check if the inner type `T` implements `PartialOrd`. Since this is a generic type, we use compile-time evaluation of `SHAPE`:

```rust
partial_ord: if T::SHAPE.vtable.partial_ord.is_some() {
    Some(|a, b| {
        let a = unsafe { a.as_ref::<[T; 1]>() };
        let b = unsafe { b.as_ref::<[T; 1]>() };
        unsafe {
            (T::SHAPE.vtable.partial_ord.unwrap_unchecked())(
                OpaqueConst::from_ref(&a[0]),
                OpaqueConst::from_ref(&b[0]),
            )
        }
    })
} else {
    None
}
```

Here's what's happening:
1. We check if `T::SHAPE.vtable.partial_ord` is `Some`, which tells us if `T` implements `PartialOrd`
2. If it does, we provide a `partial_ord` implementation that:
   - Extracts arrays from opaque pointers
   - Gets the first element from each array
   - Delegates to the inner type's `partial_ord` implementation
3. If `T` doesn't implement `PartialOrd`, we set `partial_ord` to `None`

### Non-Generic Type: Using `value_vtable` Macro

For non-generic types, we use the `value_vtable` macro which leverages auto-deref specialization:

```rust
partial_ord: if $crate::impls!($type_name: std::cmp::PartialOrd) {
    Some(|left, right| {
        use $crate::spez::*;
        (&&Spez(unsafe { left.as_ref::<$type_name>() }))
            .spez_partial_cmp(&&Spez(unsafe { right.as_ref::<$type_name>() }))
    })
} else {
    None
}
```

Here's what's happening:
1. The `impls!` macro uses auto-deref specialization to check if `$type_name` implements `PartialOrd`
2. If it does, we create a function that:
   - Extracts the values from opaque pointers
   - Wraps them in `Spez` (specialization helper)
   - Calls `spez_partial_cmp` which uses method resolution to pick the right implementation
3. If it doesn't implement `PartialOrd`, we set `partial_ord` to `None`

### Key Differences

These examples highlight the two approaches to specialization in the Shapely codebase:

1. **Generic approach**: Directly inspects `T::SHAPE` at compile time for trait information
2. **Non-generic approach**: Uses the `impls!` macro with auto-deref trick for specialization

Both approaches have the same goal: conditionally implement functionality based on trait implementations, but they use different mechanisms based on whether we're dealing with generic or non-generic types.

## Working with Characteristics and MarkerTraits

The `Characteristic` enum represents various traits that a type can implement, including both marker traits (like `Send`, `Sync`, `Copy`) and functionality traits (like `Debug`, `Clone`, `PartialEq`).

```rust
pub enum Characteristic {
    // Marker traits
    Send,
    Sync,
    Copy,
    Eq,
    // Functionality traits
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    Default,
}
```

### BitFlags Integration

Internally, these characteristics are represented using the `bitflags` crate, which provides an efficient way to store and manipulate sets of flags. The `MarkerTraits` type is a BitFlags representation that allows us to perform operations on sets of characteristics.

While you could manually use bitwise operations to manipulate these flags, `Characteristic` provides several helper methods that make working with them more ergonomic:

```rust
// Instead of manually combining flags like this:
let combined = MarkerTraits::from(Characteristic::Send) | MarkerTraits::from(Characteristic::Sync);

// You can use the `all` method:
let combined = Characteristic::all(&[Characteristic::Send, Characteristic::Sync]);
```

### Helper Methods

The `Characteristic` type provides several helper methods for working with sets of characteristics:

1. **Creating Sets**:
   ```rust
   // Create a set containing all specified characteristics
   let traits = Characteristic::all(&[Characteristic::Copy, Characteristic::Send, Characteristic::Sync]);
   ```

2. **Testing for Characteristics**:
   ```rust
   // Check if a set contains ANY of the specified characteristics
   if traits.any(&[Characteristic::Send, Characteristic::Sync]) {
       // At least one of Send or Sync is implemented
   }

   // Check if a set contains ALL of the specified characteristics
   if traits.all(&[Characteristic::Send, Characteristic::Sync]) {
       // Both Send and Sync are implemented
   }

   // Check if NONE of the specified characteristics are present
   if traits.none(&[Characteristic::Debug, Characteristic::Display]) {
       // Neither Debug nor Display is implemented
   }
   ```

3. **Operations on Sets**:
   ```rust
   // Union of two sets of characteristics
   let combined = traits1.union(traits2);

   // Intersection of two sets
   let common = traits1.intersection(traits2);
   ```

### Example: Improving HashMapImpl

The `HashMap<K, V>` implementation checks if keys and values implement various traits. This can be simplified using the helper methods:

```rust
// Instead of manually checking each trait like this:
if (K::SHAPE.marker_traits.contains(Characteristic::Copy) &&
    K::SHAPE.marker_traits.contains(Characteristic::Eq) &&
    K::SHAPE.marker_traits.contains(Characteristic::Hash)) {
    // ...
}

// You can use the `all` method:
if K::SHAPE.marker_traits.all(&[Characteristic::Copy, Characteristic::Eq, Characteristic::Hash]) {
    // Key type implements all required traits
}
```

Similarly, when combining marker traits from two types:

```rust
// Instead of manual union:
let combined_traits = K::SHAPE.marker_traits | V::SHAPE.marker_traits;

// You can use the union method:
let combined_traits = K::SHAPE.marker_traits.union(V::SHAPE.marker_traits);
```

These helper methods make the code more readable and less error-prone, especially when dealing with multiple characteristics at once.

## Peek and Poke: Reflection-Based Access

Shapely provides two powerful abstractions for working with values in a generic, reflection-based way:

1. **Peek**: Read-only access to inspect values
2. **Poke**: Write access to build or modify values

These APIs allow you to work with values of any type that implements `Shapely` in a uniform way, without having to write type-specific code.

### Peek: Reading Values

The `Peek` enum and related types provide a way to inspect any value that implements `Shapely`. This is useful for serialization, debug printing, and other operations that need to read data.

```rust
pub enum Peek<'mem> {
    Scalar(PeekValue<'mem>),
    List(PeekList<'mem>),
    Map(PeekMap<'mem>),
    Struct(PeekStruct<'mem>),
}
```

Each variant corresponds to a different kind of value:

- **Scalar**: Basic types like numbers, strings, booleans
- **List**: Sequential collections like arrays, vectors
- **Map**: Key-value collections like HashMaps
- **Struct**: Structs, tuple structs, and tuples

You can use `Peek` to traverse a value's structure and read its contents safely, with the borrow checker ensuring that the underlying data remains valid:

```rust
// Example: Reading a struct field
if let Peek::Struct(peek_struct) = my_peek {
    // Get field by name
    if let Some(field) = peek_struct.field("name") {
        // Process the field value
        if let Peek::Scalar(value) = field {
            // Do something with the scalar value
        }
    }
}
```

### Poke: Building Values

The `Poke` enum and related types provide a way to build or modify values of any type that implements `Shapely`. This is useful for deserialization, cloning, and other operations that need to create or modify data.

```rust
pub enum Poke<'mem> {
    Scalar(PokeValue<'mem>),
    List(PokeListUninit<'mem>),
    Map(PokeMapUninit<'mem>),
    Struct(PokeStruct<'mem>),
    Enum(PokeEnumNoVariant<'mem>),
}
```

The variants correspond to the same kinds of values as `Peek`, with the addition of `Enum` for working with enum variants.

Poke provides a safe abstraction for building values incrementally, ensuring that all required fields or elements are initialized:

```rust
// Example: Building a struct
let (poke, guard) = Poke::alloc::<MyStruct>();
let mut poke = poke.into_struct();

// Set field values by name
poke.set_by_name("foo", OpaqueConst::from_ref(&42u64))
    .unwrap();

// Set string field (needs proper lifetime management)
{
    let bar = String::from("Hello, World!");
    poke.set_by_name("bar", OpaqueConst::from_ref(&bar))
        .unwrap();
    // String has been moved, forget it to prevent double-free
    std::mem::forget(bar);
}

// Build the final value
let my_struct = poke.build::<MyStruct>(Some(guard));
```

#### Alternative: Building In-Place

```rust
// Initialize with default value first
let mut value: MyStruct = Default::default();

// Create a Poke instance from existing value
let mut poke = unsafe {
    Poke::from_opaque_uninit(OpaqueUninit::new(&mut value as *mut _), MyStruct::SHAPE)
}.into_struct();

// Mark fields as initialized (if they're valid from Default)
unsafe {
    poke.mark_initialized(0);
    poke.mark_initialized(1);
}

// Modify specific fields
poke.set_by_name("bar", OpaqueConst::from_ref(&String::from("New Value")))
    .unwrap();
std::mem::forget(String::from("New Value"));

// Complete the build in-place
poke.build_in_place();
```

### Memory Safety

Both Peek and Poke use a lifetime parameter `'mem` to ensure memory safety. This lifetime is tied to the lifetime of the underlying data, preventing use-after-free and other memory issues.

In rare cases, you might see `'mem` set to `'static`, but this should be used with extreme caution as it bypasses some of the borrow checker's safety guarantees.

### Common Uses

- **Serialization/Deserialization**: Convert between Shapely values and other formats
- **Cloning**: Create copies of values with potential modifications
- **Debug Printing**: Generate string representations of complex values
- **Generic Algorithms**: Write code that works with any Shapely type
