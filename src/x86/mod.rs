macro_rules! use_arch_x86 {
    ($($i:ident),*) => {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::{$($i,)*};

        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::{$($i,)*};
    };
}

flat_mod!(vec4);