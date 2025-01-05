use cdcl_engine::{CoreEngine, CoreEngineExplainKey, Literal, OuterEngine, TheoryTrait};
use either::Either;

use crate::{
    binary_clause_theory::{BinaryClause, BinaryClauseTheory, BinaryClauseTheoryExplainKey},
    clause_theory::{ClauseExplainKey, ClauseTheory, ClauseTrait, ClauseView},
};

#[derive(Clone, Copy)]
pub enum ExplainKey {
    CoreEngine(CoreEngineExplainKey),
    BinaryClauseTheory(<BinaryClauseTheory as TheoryTrait>::ExplainKey),
    ClauseTheory(<ClauseTheory as TheoryTrait>::ExplainKey),
}

impl From<CoreEngineExplainKey> for ExplainKey {
    fn from(explain_key: CoreEngineExplainKey) -> Self {
        return Self::CoreEngine(explain_key);
    }
}

impl From<BinaryClauseTheoryExplainKey> for ExplainKey {
    fn from(explain_key: BinaryClauseTheoryExplainKey) -> Self {
        return Self::BinaryClauseTheory(explain_key);
    }
}

impl From<ClauseExplainKey> for ExplainKey {
    fn from(explain_key: ClauseExplainKey) -> Self {
        return Self::ClauseTheory(explain_key);
    }
}

impl From<ExplainKey> for Either<ClauseExplainKey, Either<BinaryClauseTheoryExplainKey, CoreEngineExplainKey>> {
    fn from(explain_key: ExplainKey) -> Self {
        return match explain_key {
            ExplainKey::CoreEngine(k) => Either::Right(Either::Right(k)),
            ExplainKey::BinaryClauseTheory(k) => Either::Right(Either::Left(k)),
            ExplainKey::ClauseTheory(k) => Either::Left(k),
        };
    }
}

pub enum Constraint<ClauseT> {
    MonadicClause(Literal),
    BinaryClause(BinaryClause),
    Clause(ClauseT),
}

impl<ClauseT> Constraint<ClauseT>
where
    ClauseT: ClauseTrait,
{
    pub fn iter(&self) -> impl Iterator<Item = Literal> {
        match self {
            Self::MonadicClause(literal) => Either::Right(Either::Right([*literal].into_iter())),
            Self::BinaryClause(binary_clause) => Either::Right(Either::Left(binary_clause.iter())),
            Self::Clause(clause) => Either::Left(clause.iter_literals()),
        }
    }
}

impl<ClauseT> From<Either<ClauseT, Either<BinaryClause, Literal>>> for Constraint<ClauseT> {
    fn from(either: Either<ClauseT, Either<BinaryClause, Literal>>) -> Self {
        return match either {
            Either::Left(clause) => Constraint::Clause(clause),
            Either::Right(either) => match either {
                Either::Left(binary_clause) => Constraint::BinaryClause(binary_clause),
                Either::Right(monadic_clause) => Constraint::MonadicClause(monadic_clause),
            },
        };
    }
}

impl<ClauseT> From<Constraint<ClauseT>> for Either<ClauseT, Either<BinaryClause, Literal>> {
    fn from(constraint: Constraint<ClauseT>) -> Self {
        return match constraint {
            Constraint::MonadicClause(c) => Either::Right(Either::Right(c)),
            Constraint::BinaryClause(c) => Either::Right(Either::Left(c)),
            Constraint::Clause(c) => Either::Left(c),
        };
    }
}

pub fn make_constraint(mut iterator: impl Iterator<Item = Literal> + Clone) -> Constraint<impl ClauseTrait> {
    // TODO: 将来的にはちゃんとしたものを作る
    let len = iterator.clone().count() as u32;
    assert!(len != 0);
    return match len {
        1 => Constraint::MonadicClause(iterator.next().unwrap()),
        2 => Constraint::BinaryClause(BinaryClause::from_iter(iterator)),
        _ => Constraint::Clause(ClauseView::new(iterator)),
    };
}

pub type SATEngine = OuterEngine<ClauseTheory, OuterEngine<BinaryClauseTheory, CoreEngine<ExplainKey>>>;
