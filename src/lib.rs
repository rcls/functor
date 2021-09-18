#![allow(incomplete_features)]
#![feature(generic_associated_types)]
// #![feature(associated_type_defaults)]
// #![feature(associated_type_bounds)]

//use std::rc::Rc;

pub mod mapable;
pub mod derived_mapable;

pub use mapable::*;
pub use derived_mapable::*;

// /// Shorthand for the result of an into_fmap.
// type Them<T, U, F, Tag>
//    = <<F as Functor<T, Tag>>::Functor<U> as Functor<U, Tag>>::Item;

/// Trait for a Functor where mapping consumes the original.
///
/// `Self` is the type resulting from applying the functor to the type `T`.
///
/// `Tag` gives optional provision for disambiguation multiple functor types.
pub trait Functor<T, Tag = ()> {
    /// Apply the functor to a different type.
    type Functor<U> : Functor<U, Tag>;
    // where Self::Functor<T> = Self
    // where Self::Functor<U>::Functor<V> = Self::Functor<V>

    // Typically Item=T, but could e.g., be a Box or Rc.
    // type Item = T;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Functor<U>;
}

/// Trait for a Functor that also works on references.
pub trait RefFunctor<'a, T: 'a, Tag = ()> : Functor<T, Tag>
{
    fn fmap<U>(&'a self, f: impl Fn(&T) -> U) -> Self::Functor<U>;
}


/// Arrays come with a built-in implementation for Functor.  Arrays should work
/// with references also, but don't!
impl<T, const N: usize> Functor<T> for [T; N] {
    type Functor<U> = [U; N];
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> [U; N] { self.map(f) }
}

/// Pairs are functorial in both components.  Use a tag to indicate which.
pub struct Comp<const N: usize>;

/// (_, _) is functorial on .0
impl<T, B> Functor<T, Comp<0>> for (T, B) {
    type Functor<U> = (U, B);
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> (U, B) {
        (f(self.0), self.1)
    }
}

/// (_, _) is functorial on .1
impl<A, T> Functor<T, Comp<1>> for (A, T) {
    type Functor<U> = (A, U);
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> (A, U) {
        (self.0, f(self.1))
    }
}

/// (_, _) works on references.
impl<'a, A: Copy, T: 'a> RefFunctor<'a, T, Comp<1>> for (A, T) {
    fn fmap<U>(&'a self, f: impl Fn(&'a T) -> U) -> (A, U) {
        (self.0, f(&self.1)) }
}

#[test]
fn array1() {
    let v = [1, 2, 3];
    let vv = v.into_fmap(|x| x.to_string());
    assert_eq!(vv, ["1", "2", "3"]);
}
