use std::intrinsics::transmute;
x86_use!();

flat_mod!(vec3, vec4);
flat_mod!(mat2);

#[inline(always)]
pub(crate) unsafe fn _mm256_combine_pd (a: __m128d, b: __m128d) -> __m256d {
    transmute([a, b])
}

#[inline(always)]
pub(crate) unsafe fn _mm256_low_pd (a: __m256d) -> __m128d {
    _mm256_extractf128_pd(a, _MM_SHUFFLE(0, 1, 2, 3))
}

#[inline(always)]
pub(crate) unsafe fn _mm256_high_pd (a: __m256d) -> __m128d {
    _mm256_extractf128_pd(a, _MM_SHUFFLE(3, 2, 1, 0))
}

#[inline(always)]
pub(crate) unsafe fn _mm256_low_high_pd (a: __m256d) -> (__m128d, __m128d) {
    (_mm256_low_pd(a), _mm256_high_pd(a))
}

#[inline(always)]
pub(crate) unsafe fn _mm256_movehdup_pd (a: __m256d) -> __m256d {
    _mm256_shuffle_pd(a, a, _MM_SHUFFLE(3, 3, 1, 1))
}