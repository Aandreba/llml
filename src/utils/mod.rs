#[inline(always)]
pub(crate) unsafe fn copy_slice<T: Copy, const I: usize, const O: usize> (array: *const [T;I]) -> *const [T;O] {
    debug_assert!(O <= I);
    array as *const [T;O]
}

#[inline(always)]
pub(crate) unsafe fn copy_slice_w_offset<T: Copy, const I: usize, const O: usize> (array: *const [T;I], offset: usize) -> *const [T;O] {
    debug_assert!(O + offset <= I);
    (array as *const T).add(offset) as *const [T;O]
}

#[inline(always)]
pub(crate) unsafe fn copy_with_padding<T: Copy + Default, const I: usize, const O: usize> (array: *const [T;I]) -> [T;O] {
    debug_assert!(O >= I);
    let new = [T::default(); O];
    *(&new as *const [T;O] as *mut [T;I]) = *array;

    new
}