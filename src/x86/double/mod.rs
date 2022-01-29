use cfg_if::cfg_if;
flat_mod!(vec2);

cfg_if! {
    if #[cfg(all(feature = "llml_avx", target_feature = "avx"))] {
        flat_mod!(avx);
    } else {
        //flat_mod!(avx);
        flat_mod!(sse);
    }
}