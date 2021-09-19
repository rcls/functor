
use crate::{FunctorOnce, Functor, TypeMap};
use std::{boxed::Box, ops::Deref, rc::Rc};

trait Boxed<T> : From<T> + Deref<Target=T> {
    type Boxed<U> : Boxed<U>;
}

/// Disambiguating tag.
struct BoxedTag;

impl<B, T> TypeMap<T, BoxedTag> for B where B : Boxed<T> {
    type Functor<U> = B::Boxed<U>;
}

impl<'a, T: 'a, B> Functor<'a, T, BoxedTag> for B where B : Boxed<T> {
    fn fmap<U>(&'a self, mut f: impl FnMut(&T) -> U) -> B::Boxed<U> {
        f(self).into()
    }
}

impl<T> FunctorOnce<T, BoxedTag> for Box<T> {
    fn into_fmap<U>(self, mut f: impl FnMut(T) -> U) -> Box<U> {
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
