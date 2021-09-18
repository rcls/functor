
use crate::*;
use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};
use std::iter::*;

/// A DerivedMapable is like Mapable, except the member type is given
/// by a functor.
pub trait DerivedMapable<T, Tag = ()> where
    Self: IntoIterator<Item=Self::Member> + FromIterator<Self::Member>
{
    type Member: Functor<T, Tag, Item=T>;

    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : DerivedMapable<
            U, Tag, Member=<Self::Member as TypeMap<T, Tag>>::Functor<U>>;
}

pub struct Derived<Tag>(Tag);

impl<T, C, Tag> TypeMap<T, Derived<Tag>> for C
    where C: DerivedMapable<T, Tag>
{
    type Functor<U> = <C as DerivedMapable<T, Tag>>::Collection<U>;
}

impl<T, C, Tag> Functor<T, Derived<Tag>> for C
    where C: DerivedMapable<T, Tag>
{
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Functor<U> {
        let f = &f;
        self.into_iter().map(|x : C::Member| x.into_fmap(f)).collect()
    }
}

// Ugh .... coherency problems on this one!
#[cfg(disable)]
impl<'a, T: 'a, C: 'a, Tag> RefFunctor<'a, T, Derived<Tag>> for C where
    C: DerivedMapable<T, Tag>,
    // C::Member: RefFunctor<'a, T, Tag> + 'a,
    &'a C: IntoIterator<Item : Functor<&'a T, Tag>>,
{
    fn fmap<U>(&'a self, f: impl Fn(&T) -> U) -> Self::Functor<U> {
        let f = &f;
        let derived_f = |x: <&'a C as IntoIterator>::Item| { x.into_fmap(f) };
        self.into_iter().map(derived_f).collect()
    }
}


/// HashMap becomes a mapable with no change on the key type.
///
/// Unfortunately we can't turn HashSet into a Functor, because it only works on
/// a subset of types.
impl<K: Eq+Hash, T, S: BuildHasher + Default>
    DerivedMapable<T, Comp<1>> for HashMap<K, T, S>
{
    type Member = (K, T);
    type Collection<U> = HashMap<K, U, S>;
}

impl<K: Ord, T> DerivedMapable<T, Comp<1>> for BTreeMap<K, T>
{
    type Member = (K, T);
    type Collection<U> = BTreeMap<K, U>;
}

#[test]
fn hash_map_test() {
    let hm : HashMap<u32, &str>
        = [(1, "One"), (2, "Two"), (3, "Three")].into_iter().collect();
    let mapped = hm.into_fmap(|x| x == "Two");
    let expect = [(1, false), (2, true), (3, false)].into_iter().collect();
    assert_eq!(mapped, expect);
}
