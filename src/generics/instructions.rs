use cfg_if::cfg_if;

/// Compilation targets supported by LLML
pub enum TargetArch {
    X86(bool, bool),
    ARM(bool),
    WASM,
    UNKNOWN
}

impl TargetArch {
    /// Returns ```true``` if the compilation target is knowm, ```false``` otherwise
    pub const fn is_known (&self) -> bool {
        match self {
            Self::UNKNOWN => false,
            _ => true
        }
    }

    /// Returns ```true``` if the compilation target is unknowm, ```false``` otherwise
    pub const fn is_unknown (&self) -> bool {
        match self {
            Self::UNKNOWN => true,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is known and it's 64 bit, ```false``` otherwise
    pub const fn is_64_bit (&self) -> bool {
        match self {
            Self::X86(bit, _) => *bit,
            Self::ARM(bit) => *bit,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **x86** or **x86_64**, ```false``` otherwise
    pub const fn is_any_x86 (&self) -> bool {
        match self {
            Self::X86(_, _) => true,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **x86**, ```false``` otherwise
    pub const fn is_x86 (&self) -> bool {
        match self {
            Self::X86(bit, _) => !*bit,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **x86_64**, ```false``` otherwise
    pub const fn is_x86_64 (&self) -> bool {
        match self {
            Self::X86(bit, _) => *bit,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **arm** or **aarch64**, ```false``` otherwise
    pub const fn is_any_arm (&self) -> bool {
        match self {
            Self::ARM(_) => true,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **arm**, ```false``` otherwise
    pub const fn is_arm (&self) -> bool {
        match self {
            Self::ARM(bit) => !*bit,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **aarch64**, ```false``` otherwise
    pub const fn is_aarch64 (&self) -> bool {
        match self {
            Self::ARM(bit) => *bit,
            _ => false
        }
    }

    /// Returns ```true``` if compilation target is **wasm32**, ```false``` otherwise
    pub const fn is_wasm (&self) -> bool {
        match self {
            Self::WASM => true,
            _ => false
        }
    }

    /// Returns ```true``` if the crate has the **llml_avx** feature enabled, and compilation target has **AVX** enabled and is **x86** or **x86_64** , ```false``` otherwise
    pub const fn is_avx (&self) -> bool {
        match self {
            Self::X86(_, avx) => *avx,
            _ => false
        }
    }
}

/// Returns the current compilation target and some of it's features
pub const CURRENT_TARGET : TargetArch = {
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            #[cfg(target_arch = "x86")]
            let bit = false;
            #[cfg(target_arch = "x86_64")]
            let bit = true;

            #[cfg(all(feature = "llml_avx", target_feature = "avx"))]
            let avx = true;
            #[cfg(not(all(feature = "llml_avx", target_feature = "avx")))]
            let avx = false;

            TargetArch::X86(bit, avx)
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            #[cfg(target_arch = "arm")]
            let bit = false;
            #[cfg(target_arch = "aarch64")]
            let bit = true;

            TargetArch::ARM(bit)
        } else if #[cfg(target_arch = "wasm32")] {
            TargetArch::WASM
        } else {
            TargetArch::UNKNOWN
        }
    }
};