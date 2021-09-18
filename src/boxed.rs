
use crate::{Functor, RefFunctor, TypeMap};
use std::{boxed::Box, ops::Deref, ops::DerefMut, rc::Rc};

trait Boxed<T> : From<T> + Deref<Target=T> {
    type Boxed<U> : Boxed<U>;
}

/// Disambiguating tag.
struct BoxedTag;

impl<B, T> TypeMap<T, BoxedTag> for B where B : Boxed<T> {
    type Functor<U> = B::Boxed<U>;
}

impl<'a, T: 'a, B> RefFunctor<'a, T, BoxedTag> for B where B : Boxed<T> {
    fn fmap<U>(&'a self, f: impl Fn(&T) -> U) -> B::Boxed<U> {
        f(self).into()
    }
}

impl<T> Functor<T, BoxedTag> for Box<T> {
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Box<U> {
        f(*self).into()
    }
}

impl<T> Boxed<T> for Rc<T>  { type Boxed<U> = Rc<U>; }
impl<T> Boxed<T> for Box<T> { type Boxed<U> = Box<U>; }

#[test]
fn box_test() {
    let b1 = Box::new(27u32);
    let b2 = b1.fmap(|x| { Into::<f64>::into(*x) * 2.0 });
    let b3 = b1.into_fmap(|x| Into::<f64>::into(x) * 3.0);
    assert_eq!(*b2, 54.0);
    assert_eq!(*b3, 81.0);
}