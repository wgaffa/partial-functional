# Partial Functional
This is a collection of some types from functional programming
languages done in Rust.
My main goal was to learn more rust and this came out of
when I did a "partial-config" in Rust and decided to make it into a self contained
crate.

### Semigroup
A semigroup is a type that can be **combined** with another type to create a new one of the same type. The rust
number primitives like u32 under the Addition operator is an example of that, e.g 5 + 2 will give a new u32 of 7.
Some more interresting semigroups are Option<T> if T is also a semigroup as well as Result<T, E> if both T and E are semigroups.

### Monoid
A monoid is a subtrait of Semigroup with the additional property of **identity**. The **empty** identity and rusts
standard library Default trait can be interchangibly used. Any type that implements Semigroup and Default will
get a blanket implementation for Monoid.

### Examples
Some other examples than listed below can be found under the examples directory in the source crate.

Here is a very trivial example from a Leetcode problem
```rust
use partial_functional::prelude::*;

fn max_profit(prices: Vec<u32>) -> u32 {
    let mut cheapest = Min::empty();
    let mut profit = Max::empty();

    for price in prices {
        cheapest = cheapest.combine(price.into());
        profit = profit.combine(Max(price - cheapest.0));
    }

    profit.0
}
```
