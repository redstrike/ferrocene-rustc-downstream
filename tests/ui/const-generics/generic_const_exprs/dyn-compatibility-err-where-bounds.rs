#![feature(generic_const_exprs)]
#![allow(incomplete_features)]


const fn bar<T: ?Sized>() -> usize { 7 }

trait Foo {
    fn test(&self) where [u8; bar::<Self>()]: Sized;
}

impl Foo for () {
    fn test(&self) where [u8; bar::<Self>()]: Sized {}
}

fn use_dyn(v: &dyn Foo) {
    //~^ ERROR the trait `Foo` is not dyn compatible
    v.test();
}

fn main() {}

// ferrocene-annotations: fls_4ikc07mfrez5
// Object safety
