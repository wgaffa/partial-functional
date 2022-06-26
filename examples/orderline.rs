// This example was translated from an F# blog at https://fsharpforfunandprofit.com/posts/monoids-part2/
// Note that the blog is very old but the information is still good.

use partfun_derive::Semigroup;
use partial_functional::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct ProductLine {
    code: String,
    quantity: Sum<u32>,
    price: f32,
    line_total: Sum<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Semigroup)]
struct TotalLine {
    quantity: Sum<u32>,
    total: Sum<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum OrderLine {
    Product(ProductLine),
    Total(TotalLine),
    Empty,
}

// By implementing Default and Semigroup, OrderLine gets a blanket implementation of Monoid
impl Default for OrderLine {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::fmt::Display for OrderLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderLine::Product(x) => write!(
                f,
                "{} {} {} {}",
                x.code, x.quantity.0, x.price, x.line_total.0
            ),
            OrderLine::Total(x) => write!(f, "Quantity: {} Total: {}", x.quantity.0, x.total.0),
            OrderLine::Empty => write!(f, "No orders yet"),
        }
    }
}

impl Semigroup for OrderLine {
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (OrderLine::Total(a), OrderLine::Total(b)) => Self::Total(a.combine(b)),
            (OrderLine::Empty, a) | (a, OrderLine::Empty) => a,
            (OrderLine::Product(a), OrderLine::Total(b)) => Self::Total(b.combine(TotalLine {
                quantity: a.quantity,
                total: a.line_total,
            })),
            (OrderLine::Total(a), OrderLine::Product(b)) => Self::Total(a.combine(TotalLine {
                quantity: b.quantity,
                total: b.line_total,
            })),
            (OrderLine::Product(a), OrderLine::Product(b)) => Self::Total(TotalLine {
                quantity: a.quantity.combine(b.quantity),
                total: a.line_total.combine(b.line_total),
            }),
        }
    }
}

fn main() {
    let product_lines = vec![
        OrderLine::Product(ProductLine {
            code: String::from("AAA"),
            quantity: 2.into(),
            price: 9.99,
            line_total: 19.98.into(),
        }),
        OrderLine::Product(ProductLine {
            code: String::from("BBB"),
            quantity: 1.into(),
            price: 1.99,
            line_total: 1.99.into(),
        }),
        OrderLine::Product(ProductLine {
            code: String::from("CCC"),
            quantity: 3.into(),
            price: 1.33,
            line_total: 3.99.into(),
        }),
    ];

    let mut total = product_lines
        .into_iter()
        .inspect(|x| println!("{}", x))
        .fold(OrderLine::empty(), |tot, item| tot.combine(item)); // OrderLine::empty() and OrderLine::default() is interchangable

    println!("{}", total);

    let new_line = OrderLine::Product(ProductLine {
        code: "DDD".into(),
        quantity: 1.into(),
        price: 29.98.into(),
        line_total: 29.98.into(),
    });
    println!("Adding order: {new_line}");
    total = total.combine(new_line);

    println!("After adding product: {total}");
}
