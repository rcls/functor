//! Applicative functors.
//!
//! The traits actually work just derived from TypeMap rather than Functor.
//! But fmap can always be defined in terms of pure and apply!

use crate::{Functor, FunctorOnce, FunctorMut};


pub trait ApplicativeOnce<T, Tag=()> : FunctorOnce<T, Tag, Item=T> {
    fn pure_once(x:T) -> Self;

    fn call_once<A, U>(self, x: Self::Functor<A>) -> Self::Functor<U>
        where T: Fn(A) -> U, Self::Functor<A> : Clone;

    fn apply_once<U, F: Fn(T) -> U>(self, f: Self::Functor<F>)
                                    -> Self::Functor<U>
        where T: Clone, Self::Functor<F>: Clone;
}


pub trait Applicative<'a, T, Tag=()> : Functor<'a, T, Tag, Item=T> {
    // Presume we need to clone the item to use pure.
    fn pure(x : &T) -> Self where T: Clone;
    fn call<A, U>(&'a self, x: &'a Self::Functor<A>) -> Self::Functor<U>
        where T: Fn(&A) -> U;
    fn apply<U>(&'a self, f : &Self::Functor<impl Fn(&T) -> U>)
                -> Self::Functor<U>;
}


pub trait ApplicativeMut<'a, T, Tag=()> : FunctorMut<'a, T, Tag, Item=T> {
    fn mut_pure(x : &'a T) -> Self;
    fn mut_apply<U>(&mut self, f : &mut Self::Functor<impl FnMut(&mut T) -> U>)
                    -> Self::Functor<U>;

}

impl<T> ApplicativeOnce<T> for Option<T> {
    fn pure_once(x: T) -> Option<T> { Some(x) }
    fn call_once<A, B>(self, x: Option<A>) -> Option<B> where T: Fn(A) -> B {
        Some(self?(x?))
    }
    fn apply_once<U, F: Fn(T) -> U>(self, f: Option<F>) -> Option<U> {
        let s = self?;
        Some(f?(s))
    }
}

impl<'a, T: Clone> Applicative<'a, T> for Option<T> {
    fn pure(x : &T) -> Option<T> { Some(x.clone()) }
    fn call<A, U>(&'a self, x: &'a Option<A>) -> Option<U> where T: Fn(&A) -> U {
        Some(self.as_ref()?(x.as_ref()?))
    }
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

#[test]
fn apply_option() {
    assert_eq!(None.apply_once(Some(|x:u32| x)), None);
    let n : Option<fn(u32)->u32> = None;
    assert_eq!(Some(1).apply_once(n), None);
    assert_eq!(Some(3).apply_once(Some(|x| x*x)), Some(9));
}
