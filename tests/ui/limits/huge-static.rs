//@ only-64bit

// This test validates we gracefully fail computing a const or static of absurdly large size.
// The oddly-specific number is because of LLVM measuring object sizes in bits.

const HUGE_SIZE: usize = 1 << 61;

pub struct TooBigArray {
    arr: [u8; HUGE_SIZE],
}

impl TooBigArray {
    pub const fn new() -> Self {
        TooBigArray { arr: [0x00; HUGE_SIZE] }
    }
}

static MY_TOO_BIG_ARRAY_1: TooBigArray = TooBigArray::new();
//~^ ERROR too big
static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
//~^ ERROR too big

fn main() {}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
