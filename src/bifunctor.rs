
use crate::{Functor, FunctorOnce, TypeMap};

use std::marker::PhantomData;

/// Base trait for BiFunctor.  This has the mapping on types, but no
/// functionality.
///
/// `Tag` gives optional provision for disambiguating multiple functor types.
pub trait BiTypeMap<A, B, Tag = ()> {
    /// Apply the functor to a different type.
    ///
    /// This should satisfy:
    /// where Self::Functor<T> = Self
    /// where Self::Functor<U>::Functor<V> = Self::Functor<V>
    type BiFunctor<T, U> : BiTypeMap<T, U, Tag>;
}


/// Trait for a BiFunctor where mapping consumes the original.
///
/// `Self` is the type resulting from applying the functor to the type `T`.
pub trait BiFunctorOnce<A, B, Tag = ()> : BiTypeMap<A, B, Tag> {
    fn into_fmap2<T, U>(self, f: impl FnMut(A) -> T, g: impl FnMut(B) -> U)
                        -> Self::BiFunctor<T, U>;
}


/// Trait for a BiFunctor that works on references.
pub trait BiFunctor<A, B, Tag = ()> : BiTypeMap<A, B, Tag>
{
    fn fmap2<T, U>(&self, f: impl FnMut(&A) -> T, g: impl FnMut(&B) -> U)
                   -> Self::BiFunctor<T, U>;
}


/// Trait for a BiFunctor that works on mutable references.
pub trait BiFunctorMut<A, B, Tag = ()> : BiTypeMap<A, B, Tag>
{
    fn mut_fmap2<T, U>(&self,
                       f: impl FnMut(&mut A) -> T, g: impl FnMut(&mut B) -> U)
                       -> Self::BiFunctor<T, U>;
}


/// Pairs are a bifunctor.
impl<A,B> BiTypeMap<A,B> for (A,B) {
    type BiFunctor<T,U> = (T,U);
}

/// (_, _) is bifunctorial.
impl<A, B> BiFunctorOnce<A,B> for (A, B) {
    fn into_fmap2<T,U>(self, mut f: impl FnMut(A)->T, mut g: impl FnMut(B)->U)
                       -> (T, U) {
        (f(self.0), g(self.1))
    }
}

impl<A, B> BiFunctor<A, B> for (A, B) {
    fn fmap2<T,U>(&self, mut f: impl FnMut(&A)->T, mut g: impl FnMut(&B)->U)
                  -> (T, U) {
        (f(&self.0), g(&self.1))
    }
}


pub struct Proj0<B>(PhantomData<B>);
pub struct Proj1<A>(PhantomData<A>);

/// A bifunctor can be specialized to a functor on it's first argument.
impl<A, B, C: BiTypeMap<A, B>> TypeMap<A, Proj0<B>> for C {
    type Functor<T> = <Self as BiTypeMap<A, B>>::BiFunctor<T, B>;
}

/// A bifunctor can be specialized to a functor on it's first argument.
impl<A, B, C: BiFunctorOnce<A, B>> FunctorOnce<A, Proj0<B>> for C {
    fn into_fmap<T>(self, f: impl FnMut(A) -> T)
                    -> <Self as BiTypeMap<A, B>>::BiFunctor<T, B> {
        self.into_fmap2(f, |y| y)
    }
}

/// A bifunctor can be specialized to a functor on it's first argument.  Note
/// that this implementation clones the preserved data.
impl<'a, A, B: Clone, C: BiFunctor<A, B>> Functor<'a, A, Proj0<B>> for C
{
    fn fmap<T>(&self, f: impl FnMut(&A) -> T)
               -> <Self as BiTypeMap<A, B>>::BiFunctor<T, B> {
        self.fmap2(f, |y| y.clone())
    }
}


/// A bifunctor can be specialized to a functor on it's second argument.
impl<A, B, C: BiTypeMap<A, B>> TypeMap<B, Proj1<A>> for C {
    type Functor<T> = <Self as BiTypeMap<A, B>>::BiFunctor<A, T>;
}

/// A bifunctor can be specialized to a functor on it's second argument.
impl<A, B, C: BiFunctorOnce<A, B>> FunctorOnce<B, Proj1<A>> for C {
    fn into_fmap<T>(self, g: impl FnMut(B) -> T)
                    -> <Self as BiTypeMap<A, B>>::BiFunctor<A, T> {
        self.into_fmap2(|x| x, g)
    }
}

/// A bifunctor can be specialized to a functor on it's second argument.  Note
/// that this implementation clones the preserved data.
impl<'a, A: Clone, B, C: BiFunctor<A, B>> Functor<'a, B, Proj1<A>> for C
{
    fn fmap<T>(&self, g: impl FnMut(&B) -> T)
               -> <Self as BiTypeMap<A, B>>::BiFunctor<A, T> {
        self.fmap2(|x| x.clone(), g)
    }
}

pub trait BiCoherent<A, B, Tag = ()> : BiTypeMap<A, B, Tag> {
    fn cohere<S, T, U, V>(
        x : <Self::BiFunctor<S, T> as BiTypeMap<S, T, Tag>>::BiFunctor<U, V>)
        -> Self::BiFunctor<U, V>;
}


impl<A, B> BiCoherent<A, B> for (A, B) {
    fn cohere<S, T, U, V>(p : (U, V)) -> (U, V) { p }
}


mod submod {

#[test]
fn bif() {
    use super::{BiFunctor, BiFunctorOnce};

    let p: (u8, u32) = (4, 5);
    let q = p.fmap2(|x| x.to_string(), |y| y.to_string());
    let r = p.into_fmap2(|x| x*2, |y| y as f32 / 2.0);
    assert_eq!(r, (8, 2.5));
    assert_eq!(q, ("4".into(), "5".into()));
}

}
