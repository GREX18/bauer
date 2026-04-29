use bauer::Builder;

#[derive(Builder)]
#[builder(on(Vec<_> => repeat = #1))]
struct Foo {
    foo: Vec<u32>,
}

fn main() {}
