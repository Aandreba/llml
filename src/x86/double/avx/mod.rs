use std::intrinsics::transmute;
x86_use!();

flat_mod!(vec3, vec4);
flat_mod!(mat2, mat3);

#[inline(always)]
pub(crate) unsafe fn _mm256_sum_pd (lhs: __m256d) -> f64 {
    let shuf = _mm256_movehdup_pd(lhs);
    let sums = _mm256_add_pd(lhs, shuf);
    let shuf = _mm256_shuffle_pd(shuf, sums, _MM_SHUFFLE(3, 2, 3, 2));
    let sums = _mm256_add_pd(sums, shuf);
    
    _mm256_cvtsd_f64(sums)
}

#[inline(always)]
pub(crate) unsafe fn _mm256_combine_pd (a: __m128d, b: __m128d) -> __m256d {
    transmute([a, b])
}

#[inline(always)]
pub(crate) unsafe fn _mm256_low_pd (a: __m256d) -> __m128d {
    _mm256_extractf128_pd(a, _MM_SHUFFLE(0, 0, 3, 2))
}

#[inline(always)]
pub(crate) unsafe fn _mm256_high_pd (a: __m256d) -> __m128d {
    _mm256_extractf128_pd(a, _MM_SHUFFLE(0, 0, 1, 0))
}

#[inline(always)]
pub(crate) unsafe fn _mm256_low_high_pd (a: __m256d) -> (__m128d, __m128d) {
    (_mm256_low_pd(a), _mm256_high_pd(a))
}

#[inline(always)]
pub(crate) unsafe fn _mm256_movehdup_pd (a: __m256d) -> __m256d {
    _mm256_shuffle_pd(a, a, _MM_SHUFFLE(3, 3, 1, 1))
}