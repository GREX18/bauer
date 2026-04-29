#![allow(dead_code)]

use bauer::Builder;

macro_rules! flag_tests {
    ($kind: literal in mod $module: ident $($unwrap: ident)?) => {
        mod $module {
            use super::*;

            #[derive(Builder, Debug)]
            #[builder(kind = $kind)]
            struct Foo {
                #[builder(flag)]
                bar: bool,
            }

            #[test]
            fn flag_sets_true() {
                let foo: Foo = Foo::builder()
                    .bar()
                    .build()
                    $(.$unwrap())?;

                assert!(foo.bar);
            }

            #[test]
            fn flag_defaults_false() {
                let foo: Foo = Foo::builder()
                    .build()
                    $(.$unwrap())?;

                assert!(!foo.bar);
            }
        }
    };
}

// Run tests for each builder kind
flag_tests!("borrowed" in mod borrowed);
flag_tests!("owned" in mod owned);
flag_tests!("type-state" in mod type_state);