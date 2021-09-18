
use crate::*;
use std::iter::*;

/// Trait for types which are mapable via iterators.
pub trait Mapable<T> where Self: IntoIterator<Item=T> + FromIterator<T> {
    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : Mapable<U>;
}

/// Anything mapable turns into a functor using its iterators.
impl<T, C: Mapable<T>> Functor<T> for C
{
    type Value<U> = <Self as Mapable<T>>::Collection<U>;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Value<U> {
        self.into_iter().map(f).collect()
    }
}

/// Mapables can also work via references.
impl<'a, T: 'a, C: 'a + Mapable<T>> RefFunctor<'a, T> for C where
    &'a C: IntoIterator<Item=&'a T>
{
    fn fmap<U>(&'a self, f: impl FnMut(&T) -> U) -> Self::Value<U> {
        self.into_iter().map(f).collect()
    }
}


/// To turn a collection class into a Mapable (and hence a Functor) we just
/// need to tie together all the intances.
impl<T> Mapable<T> for Vec<T> {
    type Collection<U> = Vec<U>;
}
