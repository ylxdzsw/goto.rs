#![feature(llvm_asm)]

use goto::{goto, label};

fn main() {
    #[allow(unused_assignments)]
    let mut a = 3;

    unsafe {
        goto!(1f); // b,f indicates the direction as in assembly.
        a = 4;
        label!(1);
        println!("{}", a)
    }
}