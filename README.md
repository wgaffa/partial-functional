# Partial Functional
This is a collection of some types from functional programming
languages done in Rust.
My main goal was to learn more rust and this came out of
when I did a "partial-config" in Rust and decided to make it into a self contained
crate.

## The Traits
### Semigroup
The
Semigroup defines a type that can be **combined** with another type of itself to
produce a new one of the same type.

### Monoid
A Monoid is a subtrait of Semigroup
and adds the **identity** aspect the type. An **identity** is like std::default::Default
and infact, any type that implements (Default + Semigroup) is also a Monoid.

### HKT
This is a lightweight Higher Kind Type (HKT). (This
was taken from [Functional Programming Jargon in Rust](https://functional.works-hub.com/learn/functional-programming-jargon-in-rust-1b555).

### Functor
A functor that takes an function and passes it's underlying type to that function. This is the same as
calling map(func) on an Option or Result type. This only unifies that behaviour under one trait.

## The Types
### Option (Semigroup)
If the underlying type is a Semigroup then this will perform the **combine** method on the left and right side
if both are of the Some variant. Otherwise it returns the only Some variant or None if both are None.

### Result (Semigroup)
Returns the right side if the left is an Err variant. Otherwise returns the left side.

### Ordering (Semigroup)

### Any & All

### First & Last

### Sum & Product
