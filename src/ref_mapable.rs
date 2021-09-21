//! Trait for collection types where references are IntoIterator and
//! FromIterator can be turned into Applicative generically.
//!
//! The RefMapable trait does the gory glue, it's implementations just need to
//! specify the set of types to use.

use crate::{Applicative, Mapable, Mapped, RefIntoIterator};

use std::collections::{LinkedList, VecDeque};

pub trait RefMapable<'a, T: 'a> : Mapable<T> + RefIntoIterator<'a>
{
    type RefColl<'b, U: 'b>: RefMapable<'b, U>;
    // This should be implemented as the identity function!
    fn inject<'b, U: 'b>(x: &'b Self::Collection<U>)
                         -> &'b Self::RefColl<'b, U>;
}

impl<'a, T: 'a, C: 'a + RefMapable<'a, T>> Applicative<'a, T, Mapped> for C
{
    fn pure(x: &T) -> C where T: Clone {
        std::iter::once(x.clone()).collect()
    }
    fn call<A, U>(&'a self, x: &C::Collection<A>) -> C::Collection<U>
        where T: Fn(&A) -> U
    {
        let x = Self::inject(x);
        self.ref_into_iter()
            .flat_map(|f| x.ref_into_iter().map(f))
            .collect()
    }
    fn apply<U>(&'a self, f: &C::Collection<impl Fn(&T) -> U>)
                -> C::Collection<U>
    {
        let f = Self::inject(f);
        self.ref_into_iter()
            .flat_map(move |x| f.ref_into_iter().map(|g| g(x)))
            .collect()
    }
}

impl<'a, T: 'a> RefMapable<'a, T> for Vec<T> {
    type RefColl<'b, U: 'b> = Vec<U>;
    fn inject<'b, U: 'b>(x : &'b Vec<U>) -> &'b Vec<U> { x }
}
impl<'a, T: 'a> RefMapable<'a, T> for LinkedList<T> {
    type RefColl<'b, U: 'b> = LinkedList<U>;
    fn inject<'b, U: 'b>(x : &'b LinkedList<U>) -> &'b LinkedList<U> { x }
}
impl<'a, T: 'a> RefMapable<'a, T> for VecDeque<T> {
    type RefColl<'b, U: 'b> = VecDeque<U>;
    fn inject<'b, U: 'b>(x : &'b VecDeque<U>) -> &'b VecDeque<U> { x }
}
