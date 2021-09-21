//! Trait for collection types that are IntoIterator and FromIterator.  These
//! can be turned into Functors generically.
//!
//! The Mapable trait does the gory glue, it's implementations just need to
//! specify the set of types to use.

use crate::{ApplicativeOnce, FunctorOnce, Functor, FunctorMut, RefIntoIterator,
            TypeMap};
use std::iter::FromIterator;

use std::collections::{LinkedList, VecDeque};

/// Trait for types which are mapable via iterators.
pub trait Mapable<T> : FromIterator<T> + IntoIterator<Item = T> where
{
    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : Mapable<U>;
}


/// Marker to disambiguate the blanket implementation of Functor for Mapables.
pub struct Mapped;

impl<T, C: Mapable<T>> TypeMap<T, Mapped> for C {
    type Functor<U> = C::Collection<U>;
}

/// Anything mapable turns into a functor using its iterators.
impl<T, C: Mapable<T>> FunctorOnce<T, Mapped> for C
{
    fn fmap_once<U>(self, f: impl FnMut(T) -> U) -> C::Collection<U> {
        self.into_iter().map(f).collect()
    }
}

/// Mapables can also work via references.
impl<'a, T: 'a, C: 'a + Mapable<T>> Functor<'a, T, Mapped> for C
    where C: RefIntoIterator<'a>
{
    fn fmap<U>(&'a self, f: impl FnMut(&T) -> U) -> C::Collection<U> {
        self.ref_into_iter().map(f).collect()
    }
}


impl<'a, T: 'a, C: 'a + Mapable<T>> FunctorMut<'a, T, Mapped> for C
    where &'a mut C: IntoIterator<Item=&'a mut T>
{
    fn fmap_mut<U>(&'a mut self, f: impl FnMut(&mut T) -> U) -> C::Collection<U> {
        self.into_iter().map(f).collect()
    }
}

impl<T, C: Mapable<T>> ApplicativeOnce<T, Mapped> for C
{
    fn pure_once(x: T) -> C {
        std::iter::once(x).collect()
    }
    fn call_once<A, B>(self, x: C::Collection<A>) -> C::Collection<B>
        where C::Collection<A>: Clone, T: Fn(A) -> B
    {
        self.into_iter()
            .flat_map(|f| x.clone().into_iter().map(f))
            .collect()
    }
    fn apply_once<U, F: Fn(T) -> U>(self, f: C::Collection<F>)
                                    -> Self::Functor<U>
        where T: Clone, C::Collection<F>: Clone
    {
        self.into_iter()
            .flat_map(|x| f.clone().into_iter().map(move |g| g(x.clone())))
            .collect()
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
