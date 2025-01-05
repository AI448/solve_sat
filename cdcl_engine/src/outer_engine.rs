use either::Either;

use crate::{
    engine::{EngineAddConstraintTrait, EngineTrait},
    theory::{TheoryAddConstraintTrait, TheoryTrait},
    types::{Boolean, Literal, PropagationResult, Reason},
};

#[derive(Clone)]
pub struct OuterEngine<TheoryT, InnerEngineT>
where
    TheoryT: TheoryTrait,
    InnerEngineT: EngineTrait,
{
    theory: TheoryT,
    inner_engine: InnerEngineT,
    number_of_propagateds: u32,
}

impl<TheoryT, InnerEngineT> OuterEngine<TheoryT, InnerEngineT>
where
    TheoryT: TheoryTrait,
    InnerEngineT: EngineTrait,
{
    pub fn new(theory: TheoryT, inner_engine: InnerEngineT) -> Self {
        Self { theory: theory, inner_engine: inner_engine, number_of_propagateds: 0 }
    }
}

impl<TheoryT, InnerEngineT> EngineTrait for OuterEngine<TheoryT, InnerEngineT>
where
    TheoryT: TheoryTrait,
    InnerEngineT: EngineTrait,
    InnerEngineT::CompositeExplainKey: From<TheoryT::ExplainKey>,
{
    type CompositeExplainKey = InnerEngineT::CompositeExplainKey;
    type ExplainKey = Either<TheoryT::ExplainKey, InnerEngineT::ExplainKey>;
    type ExplanationConstraint<'a>
        = Either<TheoryT::ExplanationConstraint<'a>, InnerEngineT::ExplanationConstraint<'a>>
    where
        Self: 'a;
    type Summary = (TheoryT::Summary, InnerEngineT::Summary);

    #[inline(always)]
    fn number_of_variables(&self) -> u32 {
        return self.inner_engine.number_of_variables();
    }

    #[inline(always)]
    fn number_of_assigneds(&self) -> u32 {
        return self.inner_engine.number_of_assigneds();
    }

    #[inline(always)]
    fn current_decision_level(&self) -> u32 {
        return self.inner_engine.current_decision_level();
    }

    #[inline(always)]
    fn is_assigned(&self, index: u32) -> bool {
        return self.inner_engine.is_assigned(index);
    }

    #[inline(always)]
    fn is_true(&self, literal: Literal) -> bool {
        return self.inner_engine.is_true(literal);
    }

    #[inline(always)]
    fn is_false(&self, literal: Literal) -> bool {
        return self.inner_engine.is_false(literal);
    }

    #[inline(always)]
    fn get_value(&self, index: u32) -> Boolean {
        return self.inner_engine.get_value(index);
    }

    #[inline(always)]
    fn get_decision_level(&self, index: u32) -> u32 {
        return self.inner_engine.get_decision_level(index);
    }

    #[inline(always)]
    fn get_assignment_order(&self, index: u32) -> u32 {
        return self.inner_engine.get_assignment_order(index);
    }

    #[inline(always)]
    fn get_reason(&self, index: u32) -> Option<Reason<Self::CompositeExplainKey>> {
        return self.inner_engine.get_reason(index);
    }

    #[inline(always)]
    fn get_assignment_order_range(&self, decision_level: u32) -> std::ops::Range<u32> {
        return self.inner_engine.get_assignment_order_range(decision_level);
    }

    #[inline(always)]
    fn get_assignment(&self, assignment_order: u32) -> Literal {
        return self.inner_engine.get_assignment(assignment_order);
    }

    fn add_variable(&mut self, initial_value: Boolean) {
        self.inner_engine.add_variable(initial_value);
        self.theory.add_variable();
    }

    fn assign(
        &mut self,
        literal: Literal,
        reason: Reason<Self::CompositeExplainKey>,
    ) -> PropagationResult<Self::CompositeExplainKey> {
        let inner_propagation_result = self.inner_engine.assign(literal, reason);
        if inner_propagation_result.is_conflict() {
            return inner_propagation_result;
        } else {
            return self.propagate();
        }
    }

    #[inline(always)]
    fn explain(&self, explain_key: Self::ExplainKey) -> Self::ExplanationConstraint<'_> {
        return match explain_key {
            Either::Left(theory_explain_key) => Either::Left(self.theory.explain(theory_explain_key)),
            Either::Right(inner_explain_key) => Either::Right(self.inner_engine.explain(inner_explain_key)),
        };
    }

    fn backjump(&mut self, backjump_level: u32) -> impl Iterator<Item = Literal> {
        let backjump_assignment_order = self.inner_engine.get_assignment_order_range(backjump_level).end;
        debug_assert!(backjump_assignment_order <= self.number_of_propagateds);
        let unassigning_literals = (backjump_assignment_order..self.number_of_propagateds)
            .rev()
            .map(|assignment_order| self.inner_engine.get_assignment(assignment_order));
        self.theory.unassign(unassigning_literals);
        self.number_of_propagateds = backjump_assignment_order;
        return self.inner_engine.backjump(backjump_level);
    }

    fn reduce_constraints(&mut self) {
        self.theory.reduce_constraints();
        self.inner_engine.reduce_constraints();
    }

    fn summary(&self) -> Self::Summary {
        return (self.theory.summary(), self.inner_engine.summary());
    }
}

impl<TheoryT, InnerEngineT, ThoeryConstraintT, InnerEngineConstraintT>
    EngineAddConstraintTrait<Either<ThoeryConstraintT, InnerEngineConstraintT>> for OuterEngine<TheoryT, InnerEngineT>
where
    TheoryT: TheoryTrait + TheoryAddConstraintTrait<ThoeryConstraintT>,
    InnerEngineT: EngineTrait + EngineAddConstraintTrait<InnerEngineConstraintT>,
    InnerEngineT::CompositeExplainKey: From<TheoryT::ExplainKey>,
{
    fn add_constraint(
        &mut self,
        constraint: Either<ThoeryConstraintT, InnerEngineConstraintT>,
        is_learnt: bool,
    ) -> PropagationResult<Self::CompositeExplainKey> {
        let result = match constraint {
            Either::Left(theory_constraint) => {
                self.theory.add_constraint(theory_constraint, is_learnt, &mut self.inner_engine)
            }
            Either::Right(inner_constraint) => self.inner_engine.add_constraint(inner_constraint, is_learnt),
        };
        if result.is_conflict() {
            return result;
        } else {
            return self.propagate();
        }
    }
}

impl<TheoryT, InnerEngineT> OuterEngine<TheoryT, InnerEngineT>
where
    TheoryT: TheoryTrait,
    InnerEngineT: EngineTrait,
    InnerEngineT::CompositeExplainKey: From<TheoryT::ExplainKey>,
{
    fn propagate(&mut self) -> PropagationResult<<Self as EngineTrait>::CompositeExplainKey> {
        while self.number_of_propagateds < self.inner_engine.number_of_assigneds() {
            let literal = self.inner_engine.get_assignment(self.number_of_propagateds);
            debug_assert!(
                self.inner_engine.get_decision_level(literal.index()) == self.inner_engine.current_decision_level()
            );
            self.number_of_propagateds += 1;
            let result = self.theory.assign(literal, &mut self.inner_engine);
            if result.is_conflict() {
                return result;
            }
        }
        return PropagationResult::Noconflict;
    }
}
