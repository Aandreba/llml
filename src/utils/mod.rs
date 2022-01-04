#[inline(always)]
pub(crate) unsafe fn copy_slice<T: Copy, const I: usize, const O: usize> (array: &[T;I]) -> [T;O] {
    debug_assert!(O <= I);
    *(array as *const [T;I] as *const [T;O])
}

#[inline(always)]
pub(crate) unsafe fn copy_slice_w_offset<T: Copy, const I: usize, const O: usize> (array: &[T;I], offset: usize) -> [T;O] {
    debug_assert!(O + offset <= I);
    *((array as *const [T;I] as *const T).add(offset) as *const [T;O])
}

#[inline(always)]
pub(crate) unsafe fn copy_with_padding<T: Copy + Default, const I: usize, const O: usize> (array: [T;I]) -> [T;O] {
    debug_assert!(O >= I);
    let new = [T::default(); O];
    *(&new as *const [T;O] as *mut [T;I]) = array;

    new
}