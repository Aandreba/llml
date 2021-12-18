use std::{iter::TrustedLen, ops::{Try, Residual, FromResidual, ControlFlow}, mem::{self, MaybeUninit}, sync::{Arc, Mutex}, alloc::{Layout, alloc, self}, borrow::BorrowMut, fmt::Debug, ptr::eq};
use rayon::iter::{ParallelIterator, IntoParallelIterator};
use super::never_short::NeverShortCircuit;

pub fn empty_array<T> () -> [T;0] {
    []
}

pub unsafe fn malloc<T> () -> T {
    let layout = Layout::new::<T>();
    *(alloc(layout) as *const T)
}

pub unsafe fn allocate_array<T: Copy, const N: usize> () -> [T;N] {
    let layout = Layout::new::<[T;N]>();
    unsafe { *(alloc(layout) as *const [T;N]) }
}

pub fn build_array<T: Copy, F: Fn(usize) -> T, const N: usize> (expr: F) -> [T;N] {
    let mut map = (0..N).into_iter().map(expr);
    unsafe {
        collect_into_array_unchecked(&mut map)
    }
}

pub fn array_map_mt<T: Send + Sync + Copy, U: Copy + Send + Sync, F: Fn(T) -> U, const N: usize> (array: [T;N], map: F) -> [U;N] where F: Send + Sync {
    build_array_mt(|i| map(array[i]))
}

pub fn build_array_mt <T: Copy + Send + Sync, F: Fn(usize) -> T, const N: usize> (expr: F) -> [T;N] where F: Sync + Send {
    let ptr;
    unsafe {
        ptr = allocate_array();
    }

    let arc = Arc::new(Mutex::new(ptr));
    (0..N).into_par_iter()
        .for_each_with(arc.clone(), |array, i| {
            let exp = expr(i);
            let mut lock = array.lock().unwrap();
            lock[i] = exp;
    });
    
    let lock = arc.lock().unwrap();
    *lock
}  

pub unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [I::Item; N]
where
I: Iterator + TrustedLen,

{
    let mut map = iter.map(NeverShortCircuit);

    // SAFETY: The same safety considerations w.r.t. the iterator length
    // apply for `try_collect_into_array_unchecked` as for
    // `collect_into_array_unchecked`
    match unsafe { try_collect_into_array_unchecked(&mut map) } {
        NeverShortCircuit(array) => array,
    }
}

unsafe fn try_collect_into_array_unchecked<I, T, R, const N: usize>(iter: &mut I) -> R::TryType
where
    // Note: `TrustedLen` here is somewhat of an experiment. This is just an
    // internal function, so feel free to remove if this bound turns out to be a
    // bad idea. In that case, remember to also remove the lower bound
    // `debug_assert!` below!
    I: Iterator + TrustedLen,
    I::Item: Try<Output = T, Residual = R>,
    R: Residual<[T; N]>,
{
    debug_assert!(N <= iter.size_hint().1.unwrap_or(usize::MAX));
    debug_assert!(N <= iter.size_hint().0);

    // SAFETY: covered by the function contract.
    unsafe { try_collect_into_array(iter).unwrap_unchecked() }
}

fn try_collect_into_array<I, T, R, const N: usize>(iter: &mut I) -> Option<R::TryType>
where
    I: Iterator,
    I::Item: Try<Output = T, Residual = R>,
    R: Residual<[T; N]>,
{
    if N == 0 {
        // SAFETY: An empty array is always inhabited and has no validity invariants.
        return unsafe { Some(Try::from_output(mem::zeroed())) };
    }

    struct Guard<'a, T, const N: usize> {
        array_mut: &'a mut [MaybeUninit<T>; N],
        initialized: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            debug_assert!(self.initialized <= N);

            // SAFETY: this slice will contain only initialized objects.
            unsafe {
                std::ptr::drop_in_place(MaybeUninit::slice_assume_init_mut(
                    &mut self.array_mut.get_unchecked_mut(..self.initialized),
                ));
            }
        }
    }

    let mut array = MaybeUninit::uninit_array::<N>();
    let mut guard = Guard { array_mut: &mut array, initialized: 0 };

    while let Some(item_rslt) = iter.next() {
        let item = match item_rslt.branch() {
            ControlFlow::Break(r) => {
                return Some(FromResidual::from_residual(r));
            }
            ControlFlow::Continue(elem) => elem,
        };

        // SAFETY: `guard.initialized` starts at 0, is increased by one in the
        // loop and the loop is aborted once it reaches N (which is
        // `array.len()`).
        unsafe {
            guard.array_mut.get_unchecked_mut(guard.initialized).write(item);
        }
        guard.initialized += 1;

        // Check if the whole array was initialized.
        if guard.initialized == N {
            mem::forget(guard);

            // SAFETY: the condition above asserts that all elements are
            // initialized.
            let out = unsafe { MaybeUninit::array_assume_init(array) };
            return Some(Try::from_output(out));
        }
    }

    // This is only reached if the iterator is exhausted before
    // `guard.initialized` reaches `N`. Also note that `guard` is dropped here,
    // dropping all already initialized elements.
    None
}