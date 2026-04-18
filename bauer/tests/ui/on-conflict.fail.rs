use bauer::Builder;

#[derive(Builder)]
#[builder(on(Vec<_> => repeat))]
struct Foo {
    #[builder(default)]
    foo: Vec<u32>,
}

fn main() {}
