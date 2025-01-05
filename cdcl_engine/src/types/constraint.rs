use super::Literal;

pub trait ClauseTrait {
    fn len(&self) -> u32;
    fn iter_literals(&self) -> impl Iterator<Item = Literal> + '_;
}

pub struct ClauseView<IteratorT>
where
    IteratorT: Iterator<Item = Literal> + Clone,
{
    iterator: IteratorT,
}

impl<IteratorT> ClauseView<IteratorT>
where
    IteratorT: Iterator<Item = Literal> + Clone,
{
    pub fn new(iterator: IteratorT) -> Self {
        Self { iterator: iterator }
    }
}

impl<IteratorT> ClauseTrait for ClauseView<IteratorT>
where
    IteratorT: Iterator<Item = Literal> + Clone,
{
    fn len(&self) -> u32 {
        self.iterator.clone().count() as u32
    }

    fn iter_literals(&self) -> impl Iterator<Item = Literal> + '_ {
        self.iterator.clone()
    }
}

#[derive(Clone, Copy)]
pub struct BinaryClause {
    literals: [Literal; 2],
}

impl BinaryClause {
    pub fn new(l0: Literal, l1: Literal) -> Self {
        Self { literals: [l0, l1] }
    }

    pub fn try_from_clause<ClauseT>(clause: &ClauseT) -> Option<Self>
    where
        ClauseT: ClauseTrait,
    {
        if clause.len() == 2 {
            let mut iter = clause.iter_literals();
            let l0 = iter.next().unwrap();
            let l1 = iter.next().unwrap();
            debug_assert!(iter.next().is_none());
            return Some(Self::new(l0, l1));
        } else {
            return None;
        }
    }
}

impl From<BinaryClause> for [Literal; 2] {
    fn from(binary_clause: BinaryClause) -> Self {
        return binary_clause.literals;
    }
}

impl ClauseTrait for BinaryClause {
    fn len(&self) -> u32 {
        2
    }

    fn iter_literals(&self) -> impl Iterator<Item = Literal> + '_ {
        self.literals.iter().cloned()
    }
}

#[derive(Clone, Copy)]
pub struct MonadicClause {
    literal: Literal,
}

impl MonadicClause {
    pub fn new(literal: Literal) -> Self {
        Self { literal: literal }
    }

    pub fn literal(&self) -> Literal {
        return self.literal;
    }
}

impl ClauseTrait for MonadicClause {
    fn len(&self) -> u32 {
        return 1;
    }
    fn iter_literals(&self) -> impl Iterator<Item = Literal> + '_ {
        return [self.literal].into_iter();
    }
}
