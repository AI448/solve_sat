use crate::{
    engine::EngineTrait,
    types::{Literal, PropagationResult},
};

pub trait TheoryTrait {
    type ExplainKey;
    type ExplanationConstraint<'a>
    where
        Self: 'a;
    type Summary;

    fn add_variable(&mut self);

    fn assign<EngineT>(
        &mut self,
        literal: Literal,
        engine: &mut EngineT,
    ) -> PropagationResult<EngineT::CompositeExplainKey>
    where
        EngineT: EngineTrait,
        EngineT::CompositeExplainKey: From<Self::ExplainKey>;

    fn explain(&self, explain_key: Self::ExplainKey) -> Self::ExplanationConstraint<'_>;

    fn unassign(&mut self, unassigned_literals: impl Iterator<Item = Literal>);

    fn reduce_constraints(&mut self);

    fn summary(&self) -> Self::Summary;
}

pub trait TheoryAddConstraintTrait<ConstraintT>: TheoryTrait {
    fn add_constraint<EngineT: EngineTrait>(
        &mut self,
        constraint: ConstraintT,
        is_learnt: bool,
        engine: &mut EngineT,
    ) -> PropagationResult<EngineT::CompositeExplainKey>
    where
        EngineT::CompositeExplainKey: From<Self::ExplainKey>;
}
