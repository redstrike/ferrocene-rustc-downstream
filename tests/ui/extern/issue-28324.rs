extern "C" {
    static error_message_count: u32;
}

pub static BAZ: u32 = *&error_message_count;
//~^ ERROR use of extern static is unsafe and requires
//~| ERROR cannot access extern static

fn main() {}

// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
