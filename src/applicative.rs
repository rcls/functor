//! Applicative functors.
//!
//! The traits actually work just derived from TypeMap rather than Functor.
//! But fmap can always be defined in terms of pure and apply!

use crate::{Functor, FunctorOnce, FunctorMut};

trait ApplicativeInto<T, Tag=()> : FunctorOnce<T, Tag, Item=T> {
    fn into_pure(x:T) -> Self;
    fn into_apply<U>(self, f : Self::Functor<impl FnMut(T) -> U>)
                     -> Self::Functor<U>;
}

trait Applicative<'a, T, Tag=()> : Functor<'a, T, Tag, Item=T> {
    fn pure(x : &T) -> Self;
    fn apply<U>(&self, f : &Self::Functor<impl Fn(&T) -> U>)
                -> Self::Functor<U>;
}

trait ApplicativeMut<'a, T, Tag=()> : FunctorMut<'a, T, Tag, Item=T> {
    fn mut_pure(x : &'a T) -> Self;
    fn mut_apply<U>(&mut self, f : &mut Self::Functor<impl FnMut(&mut T) -> U>)
                    -> Self::Functor<U>;

}

impl<T> ApplicativeInto<T> for Option<T> {
    fn into_pure(x: T) -> Option<T> { Some(x) }
    fn into_apply<U>(self, f : Option<impl FnMut(T) -> U>) -> Option<U> {
        let s = self?;
        Some(f?(s))
    }
}

impl<'a, T: Clone> Applicative<'a, T> for Option<T> {
    fn pure(x : &T) -> Option<T> { Some(x.clone()) }
    fn apply<U>(&self, f: &Option<impl Fn(&T) -> U>) -> Option<U> {
        let s = self.as_ref()?;
        Some(f.as_ref()?(s))
    }
}

impl<'a, T: Clone> ApplicativeMut<'a, T> for Option<T> {
    fn mut_pure(x : &/*mut*/ T) -> Option<T> { Some(x.clone()) }
    fn mut_apply<U>(&mut self, f : &mut Option<impl FnMut(&mut T) -> U>)
                    -> Option<U> {
        let s = self.as_mut()?;
        Some(f.as_mut()?(s))
    }
}
