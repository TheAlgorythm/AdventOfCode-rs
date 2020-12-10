use std::collections::BTreeSet;

pub struct Unique<I>
where
    I: Iterator,
{
    seen: BTreeSet<I::Item>,
    underlying: I,
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.underlying.next() {
            if !self.seen.contains(&x) {
                self.seen.insert(x.clone());
                return Some(x);
            }
        }
        None
    }
}

pub trait UniqueExt: Iterator {
    fn unique(self) -> Unique<Self>
    where
        Self::Item: Ord + Clone,
        Self: Sized,
    {
        Unique {
            seen: BTreeSet::new(),
            underlying: self,
        }
    }
}

impl<I: Iterator> UniqueExt for I {}
