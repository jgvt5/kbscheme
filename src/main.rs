#[macro_use]
extern crate nom;
mod parse;

fn main() {
    println!("{:?}", parse::sexpr("(0)"));
}
