
/// Base trait for Functor.  This has the mapping on types, but no
/// functionality.
///
/// `Tag` gives optional provision for disambiguation multiple functor types.
pub trait TypeMap<T, Tag = ()> {
    /// Apply the functor to a different type.
    ///
    /// This should satisfy:
    /// where Self::Functor<T> = Self
    /// where Self::Functor<U>::Functor<V> = Self::Functor<V>
    type Functor<U> : TypeMap<U, Tag>;

    /// What is actually stored, typically Item=T, but could e.g., be a Box or
    /// Rc.
    type Item = T;
}


/// Trait for a Functor where mapping consumes the original.
///
/// `Self` is the type resulting from applying the functor to the type `T`.
pub trait FunctorOnce<T, Tag = ()> : TypeMap<T, Tag> {
    fn fmap_once<U>(self, f: impl FnMut(Self::Item) -> U) -> Self::Functor<U>;
}


/// Trait for a Functor that works on references.
pub trait Functor<'a, T: 'a, Tag = ()> : TypeMap<T, Tag> {
    fn fmap<U>(&'a self, f: impl FnMut(&Self::Item) -> U) -> Self::Functor<U>;
}

/// Trait for a Functor that works on mutable references.
pub trait FunctorMut<'a, T, Tag = ()> : TypeMap<T, Tag> {
    /// Functor map while mutating the original.
    fn fmap_mut<U>(&'a mut self, f: impl FnMut(&mut Self::Item) -> U)
                   -> Self::Functor<U>;
    // / Mutate the original but discard output.
    // fn fmutate(&mut self, f: impl FnMut(&mut Self::Item));
}


impl<T, const N: usize> TypeMap<T> for [T; N] { type Functor<U> = [U; N]; }

/// Arrays come with a built-in implementation for Functor.  Arrays should work
/// with references also, but don't!
impl<T, const N: usize> FunctorOnce<T> for [T; N] {
    fn fmap_once<U>(self, f: impl FnMut(T) -> U) -> [U; N] { self.map(f) }
}

/// Pairs are functorial in both components.  Use a tag to indicate which.
pub struct Comp0;
pub struct Comp1;

impl<T, B> TypeMap<T, Comp0> for (T, B) { type Functor<U> = (U, B); }
impl<A, T> TypeMap<T, Comp1> for (A, T) { type Functor<U> = (A, U); }

/// (_, _) is functorial on .0
impl<T, B> FunctorOnce<T, Comp0> for (T, B) {
    fn fmap_once<U>(self, mut f: impl FnMut(T) -> U) -> (U, B) {
        (f(self.0), self.1)
    }
}

/// (_, _) is functorial on .1
impl<A, T> FunctorOnce<T, Comp1> for (A, T) {
    fn fmap_once<U>(self, mut f: impl FnMut(T) -> U) -> (A, U) {
        (self.0, f(self.1))
    }
}

/// (_, _) works on references.
impl<'a, A: Copy, T: 'a> Functor<'a, T, Comp1> for (A, T) {
    fn fmap<U>(&self, mut f: impl FnMut(&T) -> U) -> (A, U) {
        (self.0, f(&self.1)) }
}

impl<'a, A: Copy, T> FunctorMut<'a, T, Comp1> for (A, T) {
    fn fmap_mut<U>(&mut self, mut f: impl FnMut(&mut T) -> U) -> (A, U) {
        (self.0, f(&mut self.1)) }
    // fn fmutate(&mut self, mut f: impl FnMut(&mut T)) {
    // f(&mut self.1) }
}

impl<T> TypeMap<T> for Option<T> {
    type Functor<U> = Option<U>;
}
impl<T> FunctorOnce<T> for Option<T> {
    fn fmap_once<U>(self, mut f: impl FnMut(T) -> U) -> Option<U> {
        Some(f(self?))
    }
}
impl<'a, T: 'a> Functor<'a, T> for Option<T> {
    fn fmap<U>(&self, mut f: impl FnMut(&T) -> U) -> Option<U> {
        Some(f(self.as_ref()?))
    }
}
impl<'a, T> FunctorMut<'a, T> for Option<T> {
    fn fmap_mut<U>(&mut self, mut f: impl FnMut(&mut T) -> U) -> Option<U> {
        Some(f(self.as_mut()?))
    }
}

/// Because we cannot force the expected type equalities, second best is to have
/// conversion functions that _should_ always be the identity in reality.
/// 2-categories are us.
///
/// The only reason to use this, is if you are writing heavily generic code.
pub trait Coherent<T, Tag = ()> : TypeMap<T, Tag, Functor<T> = Self>
{
    /// Map iterated use for Self::Functor<..> to the correct type.
    fn cohere<U, V>(x : <Self::Functor<U> as TypeMap<U, Tag>>::Functor<V>) -> Self::Functor<V>;

    // /// Map Self to the correct instance of Functor.
    //fn inject(x : Self) -> Self::Functor<T>;
}

impl<A,T> Coherent<T, Comp1> for (A,T) {
    fn cohere<U,V>(x : (A,V)) -> (A,V) { x }
    // fn inject(x : (A,T)) -> (A,T) { x }
}


#[test]
fn array1() {
    let v = [1, 2, 3];
    let vv = v.fmap_once(|x| x.to_string());
    assert_eq!(vv, ["1", "2", "3"]);
}
