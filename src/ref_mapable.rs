//! Trait for collection types where references are IntoIterator and
//! FromIterator can be turned into Applicative generically.
//!
//! The RefMapable trait does the gory glue, it's implementations just need to
//! specify the set of types to use.

use crate::{Applicative, Mapable, Mapped, RefIntoIterator};

use std::collections::{LinkedList, VecDeque};

/// RefMapable is a trait that describes what is required to implement
/// Applicable using iterators.
///
/// The actual iterator constraints are in the inherited traits Mapable and
/// RefIntoIterator.  The RefMapable trait glues these together over multiple
/// types.
pub trait RefMapable<'a, T: 'a> : 'a + Mapable<T> + RefIntoIterator<'a>
{
    /// The collection at different types should also be RefMapable.
    type RefColl<'b, U: 'b>: RefMapable<'b, U>;

    /// Rust doesn't provide a way to constain that RefColl and Collection are
    /// the same associated type.  So we provide a dummy function to help us.
    ///
    /// This should be implemented as the identity function!
    fn inject<'b, U: 'b>(x: &'b Self::Collection<U>)
                         -> &'b Self::RefColl<'b, U>;
}


impl<'a, T: 'a, C: RefMapable<'a, T>> Applicative<'a, T, Mapped> for C
{
    fn pure(x: &T) -> C where T: Clone {
        std::iter::once(x.clone()).collect()
    }

    fn lift2<U:'a, V:'a>(f: impl Fn(&'a T, &'a U) -> V,
                         a: &'a C, b: &'a C::Collection<U>)
                         -> C::Collection<V> {
        let f = &f;
        let b = Self::inject(b);
        a   .ref_into_iter()
            .flat_map(|x| b.ref_into_iter().map(move |y| f(x, y)))
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

#[test]
fn ref_app() {
    fn ten  (x: &u32) -> u32 { *x * 10 }
    fn hundy(x: &u32) -> u32 { *x * 100 }
    let v: Vec<u32> = vec![1, 2, 3];
    let f: Vec<fn(&u32) -> u32> = vec![ten, hundy];

    let fv = (&f).call(&v);
    assert_eq!(fv, [10, 20, 30, 100, 200, 300]);
    let vf = (&v).apply(&f);
    assert_eq!(vf, [10, 100, 20, 200, 30, 300]);
}
