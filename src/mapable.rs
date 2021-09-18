//! Trait for collection types that are IntoIterator and FromIterator.  These
//! can be turned into Functors generically.
//!
//! The Mapable trait does the gory glue, it's implementations just need to
//! specify the set of types to use.

use crate::{Functor, RefFunctor};
use std::iter::FromIterator;

use std::collections::{LinkedList, VecDeque};

/// Trait for types which are mapable via iterators.
pub trait Mapable<T> where Self: IntoIterator<Item=T> + FromIterator<T> {
    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : Mapable<U>;
}

/// Marker to disambiguate the blanket implementation of Functor for Mapables.
pub struct Mapped;

/// Anything mapable turns into a functor using its iterators.
impl<T, C: Mapable<T>> Functor<T, Mapped> for C
{
    type Functor<U> = <Self as Mapable<T>>::Collection<U>;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Functor<U> {
        self.into_iter().map(f).collect()
    }
}

/// Mapables can also work via references.
impl<'a, T: 'a, C: 'a + Mapable<T>> RefFunctor<'a, T, Mapped> for C where
    &'a C: IntoIterator<Item=&'a T>
{
    fn fmap<U>(&'a self, f: impl Fn(&T) -> U) -> Self::Functor<U> {
        self.into_iter().map(f).collect()
    }
}


impl<T> Mapable<T> for Vec<T>        { type Collection<U> = Vec<U>; }
impl<T> Mapable<T> for LinkedList<T> { type Collection<U> = LinkedList<U>; }
impl<T> Mapable<T> for VecDeque<T>   { type Collection<U> = VecDeque<U>; }

#[test]
fn vec1() {
    let v = vec![1, 2, 3];
    let v2 = v.fmap(|x| x + 4);
    assert_eq!(v2, [5, 6, 7]);
}
