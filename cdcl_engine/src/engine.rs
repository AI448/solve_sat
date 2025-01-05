use std::ops::Range;

use crate::types::{Boolean, Literal, PropagationResult, Reason};

// NOTE: immutable なメソッド・ Theory に見せるためのメソッド・その他のメソッドに分類して trait を分けることも考えられるが，
// 面倒な割に今のところメリットもないのでやらない
pub trait EngineTrait {
    type CompositeExplainKey: Copy;
    type ExplainKey;
    type ExplanationConstraint<'a>
    where
        Self: 'a;
    type Summary;

    const NULL_ASSIGNMENT_ORDER: u32 = u32::MAX;
    const NULL_DECISION_LEVEL: u32 = u32::MAX;

    fn number_of_variables(&self) -> u32;

    fn number_of_assigneds(&self) -> u32;

    fn current_decision_level(&self) -> u32;

    fn is_assigned(&self, index: u32) -> bool;

    fn is_true(&self, literal: Literal) -> bool;

    fn is_false(&self, literal: Literal) -> bool;

    fn get_value(&self, index: u32) -> Boolean;

    fn get_decision_level(&self, index: u32) -> u32;

    fn get_assignment_order(&self, index: u32) -> u32;

    fn get_assignment_order_range(&self, decision_level: u32) -> Range<u32>;

    fn get_reason(&self, index: u32) -> Option<Reason<Self::CompositeExplainKey>>;

    fn get_assignment(&self, assignment_order: u32) -> Literal;

    fn add_variable(&mut self, initial_value: Boolean);

    #[must_use]
    fn assign(
        &mut self,
        literal: Literal,
        reason: Reason<Self::CompositeExplainKey>,
    ) -> PropagationResult<Self::CompositeExplainKey>;

    // NOTE: 基数制約よりも一般の制約条件を扱う際には explain_conflict と explain_propagation とに分ける必要がありそう
    fn explain(&self, explain_key: Self::ExplainKey) -> Self::ExplanationConstraint<'_>;

    fn backjump(&mut self, backjump_level: u32) -> impl Iterator<Item = Literal> + '_;

    fn reduce_constraints(&mut self);

    fn summary(&self) -> Self::Summary;
}

pub trait EngineAddConstraintTrait<ConstraintT>: EngineTrait {
    #[must_use]
    fn add_constraint(
        &mut self,
        constraint: ConstraintT,
        is_learnt: bool,
    ) -> PropagationResult<Self::CompositeExplainKey>;
}
