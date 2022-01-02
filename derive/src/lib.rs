extern crate proc_macro;
use proc_macro::{TokenStream};

use quote::{quote};
use syn::{DataStruct};

include!("hypot.rs");
include!("sin_cos.rs");
include!("zero.rs");
include!("tan.rs");