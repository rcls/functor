#![allow(incomplete_features)]
#![feature(generic_associated_types)]
// #![feature(associated_type_defaults)]

//use std::rc::Rc;

use std::iter::*;

// /// Shorthand for the result of an into_fmap.
// type Them<T, U, F, Tag>
//    = <<F as Functor<T, Tag>>::Value<U> as Functor<U, Tag>>::Item;

/// Trait for a Functor where mapping consumes the original.
pub trait Functor<T, Tag = ()> {
    type Value<U> : Functor<U, Tag>;
    // where Self::Value<T> = Self
    // where Self::Value<U>::Value<V> = Self::Value<V>

    // Typically Item=T, but could e.g., be a Box or Rc.
    // type Item = T;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Value<U>;
}

/// Trait for a Functor that also works on references.
pub trait RefFunctor<'a, T: 'a, Tag = ()> : Functor<T, Tag>
{
    fn fmap<U>(&'a self, f: impl FnMut(&T) -> U) -> Self::Value<U>;
}

/// Trait for types which are mappable via iterators.
pub trait Mappable<T> where Self: IntoIterator<Item=T> + FromIterator<T> {
    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : Mappable<U>;
}

/// Anything mappable turns into a functor using its iterators.
impl<T, C: Mappable<T>> Functor<T> for C
{
    type Value<U> = <Self as Mappable<T>>::Collection<U>;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Value<U> {
        self.into_iter().map(f).collect()
    }
}

/// Mappables can also work via references.
impl<'a, T: 'a, C: 'a + Mappable<T>> RefFunctor<'a, T> for C where
    &'a C: IntoIterator<Item=&'a T>
{
    fn fmap<U>(&'a self, f: impl FnMut(&T) -> U) -> Self::Value<U> {
        self.into_iter().map(f).collect()
    }
}

/// To turn a collection class into a Mappable (and hence a Functor) we just
/// need to tie together all the intances.
impl<T> Mappable<T> for Vec<T> {
    type Collection<U> = Vec<U>;
}


/// Pairs are functorial in both components.  Use a tag to indicate which.
pub struct Comp<const N: usize>;

impl<A, T> Functor<T, Comp<1>> for (A, T) {
    type Value<U> = (A, U);
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> (A, U) {
        (self.0, f(self.1))
    }
}
impl<'a, A: Copy, T: 'a> RefFunctor<'a, T, Comp<1>> for (A, T) {
    fn fmap<U>(&'a self, mut f: impl FnMut(&'a T) -> U) -> (A, U) {
        (self.0, f(&self.1)) }
}


/// A DerivedMappable is like Mappable, except the member type is given
/// by a functor.
trait DerivedMappable<T, Tag = ()>
where Self: IntoIterator<Item=Self::Member> + FromIterator<Self::Member>
{
    type Member: Functor<T, Tag>;

    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : DerivedMappable<
            U, Tag, Member=<Self::Member as Functor<T, Tag>>::Value<U>>;
}

struct Derived<Tag>((Tag,));

impl<T, Tag, C> Functor<T, Derived<Tag>> for C
    where C: DerivedMappable<T, Tag>
{
    type Value<U> = <C as DerivedMappable<T, Tag>>::Collection<U>;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Value<U> {
        let f = &f;
        self.into_iter().map(|x : C::Member| x.into_fmap(f)).collect()
    }
}


/// HashMap becomes a mappable with no change on the key type.
///
/// Unfortunately we can't turn HashSet into a Functor, because it only works
/// on a subset of types.
use std::{collections::HashMap, hash::BuildHasher, hash::Hash};
impl<K: Eq+Hash, T, S: BuildHasher + Default>
    DerivedMappable<T, Comp<1>> for HashMap<K, T, S>
{
    type Member = (K, T);
    type Collection<U> = HashMap<K, U, S>;
}


#[cfg(test)]
pub mod test {
    use crate::*;

    pub fn id<A>(x: A) -> A { x }

    pub fn idvec<A>(x : Vec<A>) -> Vec<A> {
        x.into_fmap(id)
    }

    pub fn copyvec<A: Clone>(x : &Vec<A>) -> Vec<A> {
        x.fmap(A::clone)
    }
}

#[test]
fn vec1() {
    let v = vec![1, 2, 3];
    let v2 = v.fmap(|x| x + 4);
    assert_eq!(v2, [5, 6, 7]);
}
