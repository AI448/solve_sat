use super::index::UnsignedInt;

pub struct Array<UIntT, ValueT>
where
    UIntT: UnsignedInt,
{
    vec: Vec<ValueT>,
    phantom: std::marker::PhantomData<UIntT>,
}

impl<UIntT, ValueT> Default for Array<UIntT, ValueT>
where
    UIntT: UnsignedInt,
{
    #[inline(always)]
    fn default() -> Self {
        Array { vec: Vec::default(), phantom: std::marker::PhantomData::default() }
    }
}

impl<UIntT, ValueT> Clone for Array<UIntT, ValueT>
where
    UIntT: UnsignedInt,
    ValueT: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Array { vec: self.vec.clone(), phantom: std::marker::PhantomData::default() }
    }
}

impl<UIntT, ValueT> Array<UIntT, ValueT>
where
    UIntT: UnsignedInt,
{
    #[inline(always)]
    pub fn from_iter<I>(iterator: I) -> Self
    where
        I: Iterator<Item = ValueT>,
    {
        Array { vec: Vec::from_iter(iterator), phantom: std::marker::PhantomData::default() }
    }

    #[inline(always)]
    pub fn capacity(&self) -> UIntT {
        debug_assert!(self.vec.capacity() <= UIntT::MAX.to_usize());
        UIntT::from_usize(self.vec.capacity())
    }

    #[inline(always)]
    pub fn len(&self) -> UIntT {
        debug_assert!(self.vec.len() <= UIntT::MAX.to_usize());
        UIntT::from_usize(self.vec.len())
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[inline(always)]
    pub fn first(&self) -> Option<&ValueT> {
        self.vec.first()
    }

    #[inline(always)]
    pub fn last(&self) -> Option<&ValueT> {
        self.vec.last()
    }

    #[inline(always)]
    pub fn last_mut(&mut self) -> Option<&mut ValueT> {
        self.vec.last_mut()
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: UIntT) -> &ValueT {
        unsafe { self.vec.get_unchecked(index.to_usize()) }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, index: UIntT) -> &mut ValueT {
        unsafe { self.vec.get_unchecked_mut(index.to_usize()) }
    }

    #[inline(always)]
    pub fn into_iter(self) -> <Vec<ValueT> as IntoIterator>::IntoIter {
        self.vec.into_iter()
    }

    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<ValueT> {
        self.vec.iter()
    }

    #[inline(always)]
    pub fn iter_slice(&self, from: UIntT, to: UIntT) -> std::slice::Iter<ValueT> {
        self.vec[from.to_usize()..to.to_usize()].iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<ValueT> {
        self.vec.iter_mut()
    }

    #[inline(always)]
    pub fn reserve(&mut self, additional: UIntT) {
        self.vec.reserve(additional.to_usize());
    }

    #[inline(always)]
    pub fn reserve_exact(&mut self, additional: UIntT) {
        self.vec.reserve_exact(additional.to_usize());
    }

    #[inline(always)]
    pub fn resize_with<F>(&mut self, new_len: UIntT, f: F)
    where
        F: std::ops::FnMut() -> ValueT,
    {
        self.vec.resize_with(new_len.to_usize(), f);
    }

    #[inline(always)]
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = ValueT>,
    {
        self.vec.extend(iter);
    }

    #[inline(always)]
    pub fn truncate(&mut self, len: UIntT) {
        self.vec.truncate(len.to_usize());
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[ValueT] {
        self.vec.as_slice()
    }

    #[inline(always)]
    pub fn push(&mut self, value: ValueT) -> &mut ValueT {
        self.vec.push(value);
        unsafe { self.vec.last_mut().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<ValueT> {
        self.vec.pop()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    #[inline(always)]
    pub fn swap(&mut self, a: UIntT, b: UIntT) {
        self.vec.swap(a.to_usize(), b.to_usize());
    }

    #[inline(always)]
    pub fn swap_remove(&mut self, index: UIntT) -> ValueT {
        self.vec.swap_remove(index.to_usize())
    }

    #[inline(always)]
    pub fn sort_unstable_by_key<F, K>(&mut self, f: F)
    where
        F: std::ops::FnMut(&ValueT) -> K,
        K: std::cmp::Ord,
    {
        self.vec.sort_unstable_by_key(f);
    }

    #[inline(always)]
    pub fn sort_unstable_by<F>(&mut self, f: F)
    where
        F: std::ops::FnMut(&ValueT, &ValueT) -> std::cmp::Ordering,
    {
        self.vec.sort_unstable_by(f);
    }

    #[inline(always)]
    pub fn sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: std::ops::FnMut(&ValueT) -> K,
        K: std::cmp::Ord,
    {
        self.vec.sort_by_cached_key(f)
    }
}

impl<IndexT, ValueT> Array<IndexT, ValueT>
where
    IndexT: UnsignedInt,
    ValueT: Clone,
{
    #[inline(always)]
    pub fn resize(&mut self, new_len: IndexT, value: ValueT) {
        self.vec.resize(new_len.to_usize(), value);
    }

    #[inline(always)]
    pub fn fill(&mut self, value: ValueT) {
        self.vec.fill(value);
    }
}

impl<IndexT, ValueT> Array<IndexT, ValueT>
where
    IndexT: UnsignedInt,
    ValueT: PartialEq,
{
    #[inline(always)]
    pub fn contains(&self, x: &ValueT) -> bool {
        self.vec.contains(x)
    }
}

impl<IndexT, ValueT> std::ops::Index<IndexT> for Array<IndexT, ValueT>
where
    IndexT: UnsignedInt,
{
    type Output = ValueT;
    #[inline(always)]
    fn index(&self, index: IndexT) -> &Self::Output {
        &self.vec[index.to_usize()]
    }
}

impl<UIntT, ValueT> std::ops::IndexMut<UIntT> for Array<UIntT, ValueT>
where
    UIntT: UnsignedInt,
{
    #[inline(always)]
    fn index_mut(&mut self, index: UIntT) -> &mut Self::Output {
        &mut self.vec[index.to_usize()]
    }
}

impl<SizeT, ValueT> std::ops::Index<std::ops::Range<SizeT>> for Array<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    type Output = [ValueT];
    fn index(&self, index: std::ops::Range<SizeT>) -> &Self::Output {
        &self.vec[index.start.to_usize()..index.end.to_usize()]
    }
}

impl<SizeT, ValueT> std::ops::Index<std::ops::RangeInclusive<SizeT>> for Array<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    type Output = [ValueT];
    fn index(&self, index: std::ops::RangeInclusive<SizeT>) -> &Self::Output {
        &self.vec[index.start().to_usize()..=index.end().to_usize()]
    }
}

impl<SizeT, ValueT> std::ops::Index<std::ops::RangeTo<SizeT>> for Array<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    type Output = [ValueT];
    fn index(&self, index: std::ops::RangeTo<SizeT>) -> &Self::Output {
        &self.vec[..index.end.to_usize()]
    }
}

impl<SizeT, ValueT> std::ops::Index<std::ops::RangeToInclusive<SizeT>> for Array<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    type Output = [ValueT];
    fn index(&self, index: std::ops::RangeToInclusive<SizeT>) -> &Self::Output {
        &self.vec[..=index.end.to_usize()]
    }
}

impl<SizeT, ValueT> std::ops::Index<std::ops::RangeFrom<SizeT>> for Array<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    type Output = [ValueT];
    fn index(&self, index: std::ops::RangeFrom<SizeT>) -> &Self::Output {
        &self.vec[index.start.to_usize()..]
    }
}

impl<SizeT, ValueT> std::ops::Index<std::ops::RangeFull> for Array<SizeT, ValueT>
where
    SizeT: UnsignedInt,
{
    type Output = [ValueT];
    fn index(&self, _: std::ops::RangeFull) -> &Self::Output {
        &self.vec[..]
    }
}

impl<UIntT, ValueT> std::fmt::Debug for Array<UIntT, ValueT>
where
    UIntT: UnsignedInt,
    ValueT: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.vec, f)
    }
}
