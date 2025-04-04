# Hacking Guide to Shapely

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

3. **Alternative for generic types**: For generic types, we instead leverage the fact that `SHAPE` is a `const` associated value that can be queried in const contexts. This allows us to perform compile-time checks and conditional logic based on properties of the generic parameters.

For example, in a generic implementation like `HashMap<K, V>`, we directly access the marker traits of `K::SHAPE` and `V::SHAPE` at compile time to determine what traits to implement for the containing type.

This pattern is used throughout the codebase for various traits like `Debug`, `Display`, `Clone`, `Hash`, and more, with different specialization approaches depending on whether we're dealing with non-generic or generic types.
