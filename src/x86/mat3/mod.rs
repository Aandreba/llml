use cfg_if::cfg_if;

/*cfg_if! {
    if #[cfg(any(debug, all(target_arch = "x86_64", target_feature = "avx")))] {
        flat_mod!(avx);
    }
}*/

flat_mod!(avx);