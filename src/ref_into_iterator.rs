
/// This is a trait that encapsulates the IntoIterator requirements for various
/// implementations regarding reference types.
pub trait RefIntoIterator<'a> : IntoIterator where Self::Item : 'a {
    type RefIter : Iterator<Item = &'a Self::Item>;
    fn ref_into_iter(&'a self) -> Self::RefIter;
}

impl<'a, C: 'a + IntoIterator> RefIntoIterator<'a> for C where
    Self::Item : 'a,
    for<'b> &'b C : IntoIterator<Item = &'b Self::Item>
{
    type RefIter = <&'a C as IntoIterator>::IntoIter;
    #[inline]
    fn ref_into_iter(&'a self) -> Self::RefIter { self.into_iter() }

}
