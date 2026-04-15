use bauer::Builder;

#[derive(Builder, Debug)]
struct Foo {
    #[builder(flag)]
    bar: bool,
}

#[test]
fn flag_sets_true() {
    let foo = Foo::builder().bar().build();
    assert!(foo.bar);
}

#[test]
fn flag_defaults_false() {
    let foo = Foo::builder().build();
    assert!(!foo.bar);
}