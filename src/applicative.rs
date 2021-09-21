//! Applicative functors.
//!
//! The traits actually work just derived from TypeMap rather than Functor.
//! But fmap can always be defined in terms of pure and apply!

use crate::{Functor, FunctorOnce, FunctorMut};


pub trait ApplicativeOnce<T, Tag=()> : FunctorOnce<T, Tag, Item=T> {
    fn pure_once(x:T) -> Self;

    fn lift2_once<U, V>(f: impl Fn(T, U) -> V,
                        a: Self, b: Self::Functor<U>) -> Self::Functor<V>
        where T: Clone, Self::Functor<U>: Clone;

    fn call_once<A, U>(self, x: Self::Functor<A>) -> Self::Functor<U>
        where T: Fn(A) -> U, Self::Functor<A> : Clone;

    fn apply_once<U, F: Fn(T) -> U>(self, f: Self::Functor<F>)
                                    -> Self::Functor<U>
        where T: Clone, Self: Sized, Self::Functor<F>: Clone
    { Self::lift2_once(|x,f| f(x), self, f) }
}


pub trait Applicative<'a, T: 'a, Tag=()> : Functor<'a, T, Tag> {
    /// The T → F(T) morphism.
    ///
    /// Presume that we need to clone the item to use pure.
    fn pure(x : &T) -> Self where T: Clone;

    /// (T×U → V) × F(T) × F(U) → F(V)
    fn lift2<U: 'a, V: 'a>(f: impl Fn(&'a T, &'a U) -> V,
                           a: &'a Self, b: &'a Self::Functor<U>)
                           -> Self::Functor<V>;

    /// F(U→A) × F(U) → F(A)
    fn call<A: 'a, U: 'a>(&'a self, x: &'a Self::Functor<A>) -> Self::Functor<U>
        where T: Fn(&A) -> U {
        Self::lift2(|f,x| f(x), self, x)
    }

    /// F(T) × F(T→U) → F(U)
    fn apply<U: 'a>(&'a self, f : &'a Self::Functor<impl Fn(&T) -> U + 'a>)
                    -> Self::Functor<U> {
        Self::lift2(|x,f| f(x), self, f)
    }
}


pub trait ApplicativeMut<'a, T, Tag=()> : FunctorMut<'a, T, Tag, Item=T> {
    fn mut_pure(x : &'a T) -> Self;
    fn mut_apply<U>(&mut self, f : &mut Self::Functor<impl FnMut(&mut T) -> U>)
                    -> Self::Functor<U>;
}

impl<T> ApplicativeOnce<T> for Option<T> {
    fn pure_once(x: T) -> Option<T> { Some(x) }

    fn lift2_once<U, V>(f: impl Fn(T, U) -> V,
                        a: Option<T>, b: Option<U>) -> Option<V> {
        Some(f(a?, b?))
    }

    fn call_once<A, B>(self, x: Option<A>) -> Option<B> where T: Fn(A) -> B {
        Some(self?(x?))
    }
    fn apply_once<U, F: Fn(T) -> U>(self, f: Option<F>) -> Option<U> {
        let s = self?;
        Some(f?(s))
    }
}

impl<'a, T: 'a> Applicative<'a, T> for Option<T> {
    fn pure(x : &T) -> Option<T> where T: Clone { Some(x.clone()) }

    fn lift2<U, V>(f: impl Fn(&'a T, &'a U) -> V,
                   a: &'a Option<T>, b: &'a Option<U>) -> Option<V> {
        Some(f(a.as_ref()?, b.as_ref()?))
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
