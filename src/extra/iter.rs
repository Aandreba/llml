use std::{iter::{TrustedLen, Skip}, ops::{Deref, DerefMut}};

#[derive(Debug)]
pub struct IterSlice<I: Iterator> {
    parent: I,
    len: usize,
    idx: usize
}

impl<I: Iterator> IterSlice<I> {
    pub fn new (parent: I, len: usize) -> Self {
        Self { parent, len, idx: 0 }
    }
}

impl<I: Iterator> Iterator for IterSlice<I> {
    type Item = I::Item;

    fn next (&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None
        }

        self.idx += 1;
        self.parent.next()
    }
}

// SKIP
#[derive(Debug)]
pub struct IterJump<I: Iterator> {
    parent: I,
    at: usize,
    idx: usize
}

impl<I: Iterator> IterJump<I> {
    pub fn new (parent: I, at: usize) -> Self {
        Self { parent, at, idx: 0 }
    }
}

impl<I: Iterator> Iterator for IterJump<I> {
    type Item = I::Item;

    fn next (&mut self) -> Option<Self::Item> {
        if self.idx == self.at {
            self.idx += 1;
            self.parent.nth(1);
        }

        self.idx += 1;
        self.parent.next()
    }

    fn size_hint (&self) -> (usize, Option<usize>) {
        let parent = self.parent.size_hint();
        let min = if parent.0 == 0 { 0 } else { parent.0 - 1 };
        let max = parent.1.map(|x| if x == 0 { 0 } else { x - 1 });
        (min, max)
    }
}

unsafe impl<I: TrustedLen> TrustedLen for IterJump<I> {}

// TRUSTED SKIP
pub struct TrustedSkip<I: Iterator>(Skip<I>);

impl<I: Iterator> TrustedSkip<I> {
    pub fn new (parent: Skip<I>) -> Self {
        TrustedSkip(parent)
    }
}

impl<I: Iterator> Iterator for TrustedSkip<I>  {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

unsafe impl<I: Iterator> TrustedLen for TrustedSkip<I> {}