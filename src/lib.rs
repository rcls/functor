#![allow(incomplete_features)]
#![feature(generic_associated_types)]
// #![feature(associated_type_defaults)]

//use std::rc::Rc;

pub mod mapable;

pub use mapable::*;

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
