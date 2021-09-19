
use crate::{Coherent, Comp1, Functor, FunctorOnce, TypeMap};
use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};
use std::iter::*;

/// A DerivedMapable is like Mapable, except the member type is given
/// by a functor.
pub trait DerivedMapable<T, Tag=()> : FromIterator<Self::Member>
{
    type Member: FunctorOnce<T, Tag, Item=T> + Coherent<T, Tag>;

    // Self::Collection<U>::Collection<V> = Self::Collection<V>
    // Self = Self::Collection<T>
    type Collection<U> : DerivedMapable<
            U, Tag, Member=<Self::Member as TypeMap<T, Tag>>::Functor<U>>;
}

/// Like DerivedMapable by operating on references.
pub trait DerivedMapableRef<'a, T: 'a, Tag=()> : DerivedMapable<T, Tag> {
    type MemberRef: FunctorOnce<&'a T, Tag, Item=&'a T>;

    fn convert_ref(x : Self::MemberRef) ->
        <Self::Member as TypeMap<T, Tag>>::Functor<&'a T>;
}

pub struct Derived<Tag>(Tag);

impl<T, Tag, C: DerivedMapable<T, Tag>> TypeMap<T, Derived<Tag>> for C
{
    type Functor<U> = <C as DerivedMapable<T, Tag>>::Collection<U>;
}

impl<T, Tag, C: DerivedMapable<T, Tag>> FunctorOnce<T, Derived<Tag>> for C
    where C: IntoIterator<Item=C::Member>
{
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> Self::Functor<U> {
        let f = &f;
        self.into_iter().map(|x : C::Member| x.into_fmap(f)).collect()
    }
}


/// HashMap becomes a mapable with no change on the key type.
///
/// Unfortunately we can't turn HashSet into a Functor, because it only works on
/// a subset of types.
impl<K: Eq+Hash, T, S: BuildHasher + Default>
    DerivedMapable<T, Comp1> for HashMap<K, T, S>
{
    type Member = (K, T);
    type Collection<U> = HashMap<K, U, S>;
}

impl<'a, K: Clone + Eq + Hash+'a, T: 'a, S: BuildHasher + Default>
    DerivedMapableRef<'a, T, Comp1> for HashMap<K, T, S>
{
    type MemberRef = (&'a K, &'a T);
    #[inline]
    fn convert_ref((k, t) : (&'a K, &'a T)) -> (K, &'a T) { (k.clone(), t) }
}

impl<K: Ord, T> DerivedMapable<T, Comp1> for BTreeMap<K, T>
{
    type Member = (K, T);
    type Collection<U> = BTreeMap<K, U>;
}

impl<'a, K: Clone + Ord + 'a, T: 'a>
    DerivedMapableRef<'a, T, Comp1> for BTreeMap<K, T>
{
    type MemberRef = (&'a K, &'a T);
    #[inline]
    fn convert_ref((k, t) : (&'a K, &'a T)) -> (K, &'a T) { (k.clone(), t) }
}



impl<'a, T: 'a, Tag, C: 'a> Functor<'a, T, Derived<Tag>> for C where
    &'a C: IntoIterator<Item = C::MemberRef>,
    C : DerivedMapableRef<'a, T, Tag>,
    <C::Member as TypeMap<T, Tag>>::Functor<&'a T>: FunctorOnce<&'a T, Tag, Item=&'a T>,
{
    fn fmap<U>(&'a self, f: impl Fn(&T) -> U) -> <Self as DerivedMapable<T, Tag>>::Collection<U> {
        let f = &f;
        let ff = |p : <C::Member as TypeMap<T, Tag>>::Functor<&'a T>|
                            -> <C::Member as TypeMap<T, Tag>>::Functor<U> {
            C::Member::cohere::<&'a T, U>(p.into_fmap(f))
        };
        self.into_iter()
            .map(C::convert_ref)
            .map(ff)
            .collect()
    }
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
