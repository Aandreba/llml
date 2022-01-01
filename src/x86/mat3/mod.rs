// FOR NOW, IT SEEMS LIKE AVX IS SUPER SLOW, BEING SURPASSED EVEN BY IT'S NAIVE COUNTERPART.
// UNTIL I FIGURE OUT WHAT'S WRONG, WE'LL ONLY USE SSE

/*cfg_if! {
    if #[cfg(target_feature = "avx")] {
        flat_mod!(avx);
    } else {
        flat_mod!(sse);
    }
}*/

flat_mod!(sse);