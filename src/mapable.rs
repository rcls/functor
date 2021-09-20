//! Trait for collection types that are IntoIterator and FromIterator.  These
//! can be turned into Functors generically.
//!
//! The Mapable trait does the gory glue, it's implementations just need to
//! specify the set of types to use.

use crate::{Applicative, FunctorOnce, Functor, FunctorMut, TypeMap};
use std::iter::FromIterator;

use std::collections::{LinkedList, VecDeque};

/// Trait for types which are mapable via iterators.
pub trait Mapable<T> : FromIterator<T> + IntoIterator<Item = T> where
{
    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : Mapable<U>;
}

pub trait RefIntoIterator<'a> : IntoIterator where Self::Item : 'a {
    type RefIter : Iterator<Item = &'a Self::Item>;
    fn ref_into_iter(&'a self) -> Self::RefIter;
}

pub trait RefMapable<'a, T: 'a> : Mapable<T> + RefIntoIterator<'a>
{
    type RefColl<'b, U: 'b>: RefMapable<'b, U>;
    // This should be the identity!
    fn inject<'b, U: 'b>(x: &'b Self::Collection<U>)
                         -> &'b Self::RefColl<'b, U>;
}

impl<'a, C: 'a + IntoIterator> RefIntoIterator<'a> for C where
    Self::Item : 'a,
    for<'b> &'b C : IntoIterator<Item = &'b Self::Item>
{
    type RefIter = <&'a C as IntoIterator>::IntoIter;
    #[inline]
    fn ref_into_iter(&'a self) -> Self::RefIter {
        self.into_iter()
    }
}

/// Marker to disambiguate the blanket implementation of Functor for Mapables.
pub struct Mapped;

impl<T, C: Mapable<T>> TypeMap<T, Mapped> for C {
    type Functor<U> = C::Collection<U>;
}

/// Anything mapable turns into a functor using its iterators.
impl<T, C: Mapable<T>> FunctorOnce<T, Mapped> for C
{
    fn into_fmap<U>(self, f: impl FnMut(T) -> U) -> C::Collection<U> {
        self.into_iter().map(f).collect()
    }
}

/// Mapables can also work via references.
impl<'a, T: 'a, C: 'a + Mapable<T>> Functor<'a, T, Mapped> for C
    where &'a C: IntoIterator<Item=&'a T>
{
    fn fmap<U>(&'a self, f: impl FnMut(&T) -> U) -> C::Collection<U> {
        self.into_iter().map(f).collect()
    }
}


impl<'a, T: 'a, C: 'a + Mapable<T>> FunctorMut<'a, T, Mapped> for C
    where &'a mut C: IntoIterator<Item=&'a mut T>
{
    fn mut_fmap<U>(&'a mut self, f: impl FnMut(&mut T) -> U) -> C::Collection<U> {
        self.into_iter().map(f).collect()
    }
}

impl<'a, T: 'a, C: 'a + RefMapable<'a, T>> Applicative<'a, T, Mapped> for C
    where &'a C: IntoIterator<Item=&'a T>
{
    fn pure(x: &T) -> C where T: Clone {
        std::iter::once(x.clone()).collect()
    }
    fn apply<U>(&'a self, f: &C::Collection<impl Fn(&T) -> U>)
                -> C::Collection<U>
    {
        let f = Self::inject(f);
        self.into_iter()
            .flat_map(move |x| f.ref_into_iter().map(|g| g(x)))
            .collect()
    }
}


impl<T> Mapable<T> for Vec<T>        { type Collection<U> = Vec<U>; }
impl<T> Mapable<T> for LinkedList<T> { type Collection<U> = LinkedList<U>; }
impl<T> Mapable<T> for VecDeque<T>   { type Collection<U> = VecDeque<U>; }

impl<'a, T: 'a> RefMapable<'a, T> for Vec<T> {
    type RefColl<'b, U: 'b> = Vec<U>;
    fn inject<'b, U: 'b>(x : &'b Vec<U>) -> &'b Vec<U> { x }
}
impl<'a, T: 'a> RefMapable<'a, T> for LinkedList<T> {
    type RefColl<'b, U: 'b> = LinkedList<U>;
    fn inject<'b, U: 'b>(x : &'b LinkedList<U>) -> &'b LinkedList<U> { x }
}

#[test]
fn vec1() {
    let v = vec![1, 2, 3];
    let v2 = v.fmap(|x| x + 4);
    assert_eq!(v2, [5, 6, 7]);
}
