#![feature(llvm_asm)]

#[macro_export]
macro_rules! label {
    ( $($label:tt)* ) => {
        llvm_asm!(concat!($(stringify!($label),)* ":"))
    }
}

#[macro_export]
macro_rules! goto {
    ( $($label:tt)* ) => {
        if cfg!(target_arch="arm") || cfg!(target_arch="aarch64") {
            llvm_asm!(concat!("b ", $(stringify!($label),)*))
        } else if cfg!(target_arch="x86") || cfg!(target_arch="x86_64") {
            llvm_asm!(concat!("jmp ", $(stringify!($label),)*))
        }
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_forward() {
        #[allow(unused_assignments)]
        let mut a = 3;
        unsafe {
            goto!(1f);
            a = 4;
            label!(1);
            assert_eq!(a, 3);

            goto!(2f);
            label!(2);
            a = 4;
            assert_eq!(a, 4)
        }
    }

    #[test]
    fn global_labels() {
        #[allow(unused_assignments)]
        let mut a = 3;
        unsafe {
            goto!(__skip);
            a = 4;
            label!(__skip);
            assert_eq!(a, 3);
        }
    }

    #[test]
    fn dotted_global_labels() {
        #[allow(unused_assignments)]
        let mut a = 3;
        unsafe {
            goto!(.Lskip);
            a = 4;
            label!(.Lskip);
            assert_eq!(a, 3);
        }
    }

    #[test]
    fn branch_and_loop() {
        let mut a = 3;

        unsafe {
            label!(1);

            if a <= 3 {
                a += 1;
                goto!(2f);
            }

            if a > 6 {
                label!(2);
                assert!(a <= 6);
                goto!(1b);
            }

            assert_eq!(a, 4);
        }
    }
}
