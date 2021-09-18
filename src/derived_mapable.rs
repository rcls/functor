
use crate::*;
use std::{collections::HashMap, hash::BuildHasher, hash::Hash};
use std::iter::*;

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
