//! examples/lockall_soundness.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, dispatchers = [GPIOA])]
mod app {
    #[shared]
    struct Shared {
        a: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared { a: 0 }, Local {}, init::Monotonics())
    }

    #[task(shared = [a])]
    fn foo(mut c: foo::Context) {
        let _ = c.shared.lock(|foo::Shared { a }| {
            a // lifetime
        });
    }
}
