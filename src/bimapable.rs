//! Like mapable, but for types where the iterators are a bifunctor.
//! This is a bit gratuitous, because the only use-case I know, the bifunctor
//! is pairing.

use crate::{
    BiCoherent, BiFunctorOnce, BiTypeMap, Functor, FunctorOnce, TypeMap};

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;


pub trait BiMapable<K, T, Tag=()>
{
    type Member: BiFunctorOnce<K, T, Tag> + BiCoherent<K, T, Tag>;

    /// Note that we only need to change type on the second coordinate!
    type Collection<U> :
    FromIterator<<Self::Member as BiTypeMap<K, T, Tag>>::BiFunctor<K, U>> +
    BiMapable<K, U, Tag,
              Member=<Self::Member as BiTypeMap<K, T, Tag>>::BiFunctor<K, U>>;
}

pub struct Derived<K, Tag>(PhantomData<K>, PhantomData<Tag>);

impl<K, T, Tag, C: BiMapable<K, T, Tag>> TypeMap<T, Derived<K, Tag>> for C
{
    type Functor<U> = C::Collection<U>;
}

impl<K, T, Tag, C: BiMapable<K, T, Tag>> FunctorOnce<T, Derived<K, Tag>> for C
    where C: IntoIterator<Item=C::Member>
{
    fn into_fmap<U>(self, mut f: impl FnMut(T) -> U) -> Self::Functor<U> {
        self.into_iter().map(|x| x.into_fmap2(|k| k, &mut f)).collect()
    }
}

impl<'a, K: 'a + Clone, T: 'a, Tag, C: 'a + BiMapable<K, T, Tag>>
    Functor<'a, T, Derived<K, Tag>> for C
    where
    &'a C: IntoIterator<
        Item = <C::Member as BiTypeMap<K, T, Tag>>::BiFunctor<&'a K, &'a T>>,
    <C::Member as BiTypeMap<K, T, Tag>>::BiFunctor<&'a K, &'a T>
             : BiFunctorOnce<&'a K, &'a T, Tag>,
{
    fn fmap<U>(&'a self, mut f: impl FnMut(&T) -> U) -> C::Collection<U> {
        self.into_iter()
            .map(|v| v.into_fmap2(|k| k.clone(), &mut f))
            .map(C::Member::cohere::<&'a K, &'a T, K, U>)
            .collect()
    }
}


/// HashMap becomes a mapable with no change on the key type.
///
/// Unfortunately we can't turn HashSet into a Functor, because it only works on
/// a subset of types.
impl<K: Eq+Hash, T, S: BuildHasher + Default>
    BiMapable<K, T> for HashMap<K, T, S>
{
    type Member = (K, T);
    type Collection<U> = HashMap<K, U, S>;
}

impl<K: Ord, T> BiMapable<K, T> for BTreeMap<K, T>
{
    type Member = (K, T);
    type Collection<U> = BTreeMap<K, U>;
}


#[test]
fn hash_map_test() {
    let hm : HashMap<u32, &str>
        = [(1, "One"), (2, "Two"), (3, "Three")].into_iter().collect();
    let lengths = hm.fmap(|x| x.len());
    let expect = [(1, 3), (2, 3), (3, 5)].into_iter().collect();
    assert_eq!(lengths, expect);

    let mapped = hm.into_fmap(|x| x == "Two");
    let expect = [(1, false), (2, true), (3, false)].into_iter().collect();
    assert_eq!(mapped, expect);
}

#[test]
fn btree_test() {
    let bm : BTreeMap<u32, &str>
        = [(1, "One"), (2, "Two"), (3, "Three")].into_iter().collect();
    let mapped = bm.into_fmap(|x| x == "Two");
    let expect = [(1, false), (2, true), (3, false)].into_iter().collect();
    assert_eq!(mapped, expect);
}
