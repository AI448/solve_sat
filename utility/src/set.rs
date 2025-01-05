use crate::{Map, UnsignedInt};

#[derive(Default, Clone)]
pub struct Set<SizeT>
where
    SizeT: UnsignedInt,
{
    map: Map<SizeT, ()>,
}

impl<SizeT> Set<SizeT>
where
    SizeT: UnsignedInt,
{
    #[inline(always)]
    pub fn len(&self) -> SizeT {
        self.map.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline(always)]
    pub fn contains_key(&self, index: SizeT) -> bool {
        self.map.contains_key(index)
    }

    #[inline(always)]
    pub fn insert(&mut self, index: SizeT) {
        self.map.insert(index, ());
    }

    #[inline(always)]
    pub fn remove(&mut self, index: SizeT) -> Option<()> {
        self.map.remove(index)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &SizeT> + Clone {
        self.map.iter().map(|(index, _)| index)
    }
}

impl<IndexT> std::fmt::Debug for Set<IndexT>
where
    IndexT: UnsignedInt,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
