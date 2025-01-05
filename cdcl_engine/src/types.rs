mod boolean;
// mod constraint;
mod literal;
mod literal_array;

// use std::ops::FromResidual;

pub use boolean::Boolean;
// pub use constraint::{BinaryClause, ClauseTrait, ClauseView, MonadicClause};
pub use literal::Literal;
pub use literal_array::LiteralArray;

// use crate::throries::{binary_clause_theory::BinaryClauseExplainKey, clause_theory::ClauseExplainKey};

// pub enum CompositeConstraint<ClauseT>
// where
//     ClauseT: ClauseTrait
// {
//     MonadicClause(MonadicClause),
//     BinaryClause(BinaryClause),
//     Clause(ClauseT),
// }

// #[derive(Clone, Copy)]
// pub enum CompositeExplainKey {
//     MonadicClause(MonadicClause),
//     BinaryClause(BinaryClauseExplainKey),
//     Clause(ClauseExplainKey),
// }

// impl From<BinaryClauseExplainKey> for CompositeExplainKey {
//     fn from(binary_clause_explain_key: BinaryClauseExplainKey) -> Self {
//         Self::BinaryClause(binary_clause_explain_key)
//     }
// }

// impl From<ClauseExplainKey> for CompositeExplainKey {
//     fn from(clause_explain_key: ClauseExplainKey) -> Self {
//         Self::Clause(clause_explain_key)
//     }
// }

/// 割り当て理由
#[derive(Clone, Copy)]
pub enum Reason<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy,
{
    /// 決定
    Decision,
    /// 伝播
    Propagation {
        /// 伝播が発生した制約条件
        explain_key: CompositeExplainKeyT,
    },
}

impl<CompositeExplainKeyT> Reason<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy,
{
    #[inline(always)]
    pub fn is_decision(&self) -> bool {
        return match &self {
            Self::Decision => true,
            Self::Propagation { .. } => false,
        };
    }
    #[inline(always)]
    pub fn is_propagation(&self) -> bool {
        return match &self {
            Self::Decision => false,
            Self::Propagation { .. } => true,
        };
    }
}

#[derive(Clone)]
pub enum PropagationResult<CompositeExplainKeyT> {
    Conflict { explain_key: CompositeExplainKeyT },
    Noconflict,
}

impl<CompositeExplainKeyT> PropagationResult<CompositeExplainKeyT> {
    #[inline(always)]
    pub fn is_conflict(&self) -> bool {
        return matches!(&self, Self::Conflict { .. });
    }

    #[inline(always)]
    pub fn is_no_conflict(&self) -> bool {
        return matches!(&self, Self::Noconflict);
    }
}

// impl std::ops::FromResidual<CompositeExplainKey> for PropagationResult {
//     fn from_residual(explain_key: CompositeExplainKey) -> Self {
//         return PropagationResult::Conflict { explain_key };
//     }
// }

// impl std::ops::Try for PropagationResult {
//     type Output = ();
//     type Residual = CompositeExplainKey;
//     #[inline(always)]
//     fn from_output(_: Self::Output) -> Self {
//         return PropagationResult::Noconflict;
//     }

//     #[inline]
//     fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
//         return match self {
//             Self::Noconflict => std::ops::ControlFlow::Continue(()),
//             Self::Conflict { explain_key } => std::ops::ControlFlow::Break(explain_key),
//         };
//     }
// }
