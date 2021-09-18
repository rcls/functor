
use crate::*;
use std::{collections::HashMap, hash::BuildHasher, hash::Hash};
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


/// A DerivedMapable is like Mapable, except the member type is given
/// by a functor.
pub trait DerivedMapable<T, Tag = ()>
where Self: IntoIterator<Item=Self::Member> + FromIterator<Self::Member>
{
    type Member: Functor<T, Tag>;

    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : DerivedMapable<
            U, Tag, Member=<Self::Member as Functor<T, Tag>>::Value<U>>;
}

struct Derived<Tag>((Tag,));

impl<T, Tag, C> Functor<T, Derived<Tag>> for C
    where C: DerivedMapable<T, Tag>
{
    type Value<U> = <C as DerivedMapable<T, Tag>>::Collection<U>;

    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Value<U> {
        let f = &f;
        self.into_iter().map(|x : C::Member| x.into_fmap(f)).collect()
    }
}


/// HashMap becomes a mapable with no change on the key type.
///
/// Unfortunately we can't turn HashSet into a Functor, because it only works
/// on a subset of types.
impl<K: Eq+Hash, T, S: BuildHasher + Default>
    DerivedMapable<T, Comp<1>> for HashMap<K, T, S>
{
    type Member = (K, T);
    type Collection<U> = HashMap<K, U, S>;
}
