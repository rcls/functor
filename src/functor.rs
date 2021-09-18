
/// Base trait for Functor.  THis has the mapping on types, but no
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
pub trait Functor<T, Tag = ()> : TypeMap<T, Tag> {
    fn into_fmap<U>(self, f: impl Fn(Self::Item) -> U) -> Self::Functor<U>;
}


/// Trait for a Functor that works on references.
pub trait RefFunctor<'a, T: 'a, Tag = ()> : TypeMap<T, Tag>
{
    fn fmap<U>(&'a self, f: impl Fn(&Self::Item) -> U) -> Self::Functor<U>;
}

impl<T, const N: usize> TypeMap<T> for [T; N] { type Functor<U> = [U; N]; }

/// Arrays come with a built-in implementation for Functor.  Arrays should work
/// with references also, but don't!
impl<T, const N: usize> Functor<T> for [T; N] {
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> [U; N] { self.map(f) }
}

/// Pairs are functorial in both components.  Use a tag to indicate which.
pub struct Comp<const N: usize>;

impl<T, B> TypeMap<T, Comp<0>> for (T, B) { type Functor<U> = (U, B); }
impl<A, T> TypeMap<T, Comp<1>> for (A, T) { type Functor<U> = (A, U); }

/// (_, _) is functorial on .0
impl<T, B> Functor<T, Comp<0>> for (T, B) {
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> (U, B) {
        (f(self.0), self.1)
    }
}

/// (_, _) is functorial on .1
impl<A, T> Functor<T, Comp<1>> for (A, T) {
    fn into_fmap<U>(self, f: impl Fn(T) -> U) -> (A, U) {
        (self.0, f(self.1))
    }
}

/// (_, _) works on references.
impl<'a, A: Copy, T: 'a> RefFunctor<'a, T, Comp<1>> for (A, T) {
    fn fmap<U>(&'a self, f: impl Fn(&'a T) -> U) -> (A, U) {
        (self.0, f(&self.1)) }
}


#[test]
fn array1() {
    let v = [1, 2, 3];
    let vv = v.into_fmap(|x| x.to_string());
    assert_eq!(vv, ["1", "2", "3"]);
}
