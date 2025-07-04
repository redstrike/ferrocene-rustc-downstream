//@ run-pass
#![allow(dead_code)]
mod base {
    pub trait HasNew {
        fn new() -> Self;
    }

    pub struct Foo {
        dummy: (),
    }

    impl crate::base::HasNew for Foo {
        fn new() -> Foo {
            println!("Foo");
            Foo { dummy: () }
        }
    }

    pub struct Bar {
        dummy: (),
    }

    impl crate::base::HasNew for Bar {
        fn new() -> Bar {
            println!("Bar");
            Bar { dummy: () }
        }
    }
}

pub fn main() {
    let _f: base::Foo = base::HasNew::new();
    let _b: base::Bar = base::HasNew::new();
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
