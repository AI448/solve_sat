use std::hint::unreachable_unchecked;

use utility::Array;

use crate::{
    engine::{EngineAddConstraintTrait, EngineTrait},
    types::{Boolean, Literal, PropagationResult, Reason},
};

#[derive(Clone, Copy)]
pub struct CoreEngineExplainKey {
    literal: Literal,
}

#[derive(Default, Clone)]
pub struct CoreEngineSummary {
    pub number_of_fixed_variables: u32,
}

#[derive(Clone)]
pub struct CoreEngine<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy,
{
    states: Array<u32, State>,
    assignment_orders: Array<u32, u32>,
    assignment_stack: Array<u32, Assignment<CompositeExplainKeyT>>,
    decision_stack: Array<u32, Decision>,
    backjump_result: Array<u32, Literal>,
}

impl<CompositeExplainKeyT> Default for CoreEngine<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy,
{
    fn default() -> Self {
        Self {
            states: Array::default(),
            assignment_orders: Array::default(),
            assignment_stack: Array::default(),
            decision_stack: Array::default(),
            backjump_result: Array::default(),
        }
    }
}

impl<CompositeExplainKeyT> EngineTrait for CoreEngine<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy,
{
    type CompositeExplainKey = CompositeExplainKeyT;
    type ExplainKey = CoreEngineExplainKey;
    type ExplanationConstraint<'a>
        = Literal
    where
        Self: 'a;
    type Summary = CoreEngineSummary;

    #[inline(always)]
    fn number_of_variables(&self) -> u32 {
        return self.states.len();
    }

    #[inline(always)]
    fn number_of_assigneds(&self) -> u32 {
        return self.assignment_stack.len();
    }

    #[inline(always)]
    fn current_decision_level(&self) -> u32 {
        return self.decision_stack.len();
    }

    #[inline(always)]
    fn is_assigned(&self, index: u32) -> bool {
        return self.states[index].is_assigned();
    }

    #[inline(always)]
    fn is_true(&self, literal: Literal) -> bool {
        return self.states[literal.index()].is_assigned_to(literal.value());
    }

    #[inline(always)]
    fn is_false(&self, literal: Literal) -> bool {
        return self.states[literal.index()].is_assigned_to(!literal.value());
    }

    #[inline(always)]
    fn get_value(&self, index: u32) -> Boolean {
        return self.states[index].value();
    }

    #[inline(always)]
    fn get_assignment_order(&self, index: u32) -> u32 {
        return self.assignment_orders[index];
    }

    #[inline(always)]
    fn get_decision_level(&self, index: u32) -> u32 {
        let assignment_order = self.assignment_orders[index];
        if assignment_order == Self::NULL_ASSIGNMENT_ORDER {
            return Self::NULL_DECISION_LEVEL;
        } else {
            return self.assignment_stack[assignment_order].decision_level;
        }
    }

    #[inline(always)]
    fn get_reason(&self, index: u32) -> Option<Reason<Self::CompositeExplainKey>> {
        let assignment_order = self.assignment_orders[index];
        if assignment_order == Self::NULL_ASSIGNMENT_ORDER {
            return None;
        } else {
            return Some(self.assignment_stack[assignment_order].reason);
        }
    }

    #[inline(always)]
    fn get_assignment_order_range(&self, decision_level: u32) -> std::ops::Range<u32> {
        debug_assert!(decision_level <= self.current_decision_level());
        std::ops::Range {
            start: if decision_level == 0 { 0 } else { self.decision_stack[decision_level - 1].assignment_order },
            end: if decision_level < self.decision_stack.len() {
                self.decision_stack[decision_level].assignment_order
            } else {
                self.assignment_stack.len()
            },
        }
    }

    #[inline(always)]
    fn get_assignment(&self, assignment_order: u32) -> Literal {
        let index = self.assignment_stack[assignment_order].index;
        let value = self.states[index].value();
        return Literal::new(index, value);
    }

    #[inline(always)]
    fn add_variable(&mut self, initial_value: Boolean) {
        self.states.push(State::new(initial_value));
        self.assignment_orders.push(Self::NULL_ASSIGNMENT_ORDER);
    }

    fn assign(
        &mut self,
        literal: Literal,
        reason: Reason<Self::CompositeExplainKey>,
    ) -> PropagationResult<Self::CompositeExplainKey> {
        debug_assert!(!self.states[literal.index()].is_assigned());
        debug_assert!(self.assignment_orders[literal.index()] == Self::NULL_ASSIGNMENT_ORDER);
        let assignment_order = self.assignment_stack.len();
        if reason.is_decision() {
            self.decision_stack.push(Decision { assignment_order: assignment_order });
        }
        let decision_level = self.decision_stack.len();
        self.assignment_stack.push(Assignment {
            index: literal.index(),
            decision_level: decision_level,
            reason: reason,
        });
        self.states[literal.index()].assign(literal.value());
        self.assignment_orders[literal.index()] = assignment_order;
        return PropagationResult::Noconflict;
    }

    #[inline(always)]
    fn explain(&self, explain_key: Self::ExplainKey) -> Self::ExplanationConstraint<'_> {
        return explain_key.literal;
    }

    fn backjump(&mut self, backjump_level: u32) -> impl Iterator<Item = Literal> + '_ {
        self.backjump_result.clear();
        while self.decision_stack.len() > backjump_level {
            let assignment = self.assignment_stack.pop().unwrap();
            debug_assert!(assignment.decision_level == self.decision_stack.len());
            if assignment.reason.is_decision() {
                let decision = self.decision_stack.pop().unwrap();
                debug_assert!(decision.assignment_order == self.assignment_stack.len());
            }
            let value = self.states[assignment.index].value();
            self.states[assignment.index].unassign();
            self.assignment_orders[assignment.index] = Self::NULL_ASSIGNMENT_ORDER;
            self.backjump_result.push(Literal::new(assignment.index, value));
        }
        return self.backjump_result.iter().cloned();
    }

    fn reduce_constraints(&mut self) {
        // なにもしない
    }

    fn summary(&self) -> Self::Summary {
        return Self::Summary {
            number_of_fixed_variables: if self.decision_stack.is_empty() {
                self.assignment_stack.len()
            } else {
                self.decision_stack[0].assignment_order
            },
        };
    }
}

impl<CompositeExplainKeyT> EngineAddConstraintTrait<Literal> for CoreEngine<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy + From<Self::ExplainKey>,
{
    fn add_constraint(&mut self, literal: Literal, _is_learnt: bool) -> PropagationResult<Self::CompositeExplainKey> {
        debug_assert!(self.current_decision_level() == 0);
        debug_assert!(!self.is_false(literal));
        if !self.is_assigned(literal.index()) {
            return self.assign(literal, Reason::Propagation { explain_key: Self::ExplainKey { literal }.into() });
        } else {
            return PropagationResult::Noconflict;
        }
    }
}

#[derive(Default, Clone, Copy)]
struct State {
    bits: u8,
}

impl State {
    const VALUE_FLAG_MASK: u8 = 1;
    const ASSIGNMENT_FLAG_MASK: u8 = 2;

    #[inline(always)]
    pub fn new(initial_value: Boolean) -> Self {
        Self { bits: initial_value as u8 }
    }

    #[inline(always)]
    pub fn is_assigned(&self) -> bool {
        self.bits & Self::ASSIGNMENT_FLAG_MASK != 0
    }

    #[inline(always)]
    pub fn value(&self) -> Boolean {
        match self.bits & Self::VALUE_FLAG_MASK {
            0 => Boolean::FALSE,
            1 => Boolean::TRUE,
            _ => {
                // TODO: debug_unreachable のようなマクロを作ったほうがいい
                debug_assert!(false);
                unsafe { unreachable_unchecked() }
            }
        }
    }

    #[inline(always)]
    pub fn is_assigned_to(&self, value: Boolean) -> bool {
        self.bits == Self::ASSIGNMENT_FLAG_MASK | (value as u8)
    }

    #[inline(always)]
    pub fn assign(&mut self, value: Boolean) {
        debug_assert!(!self.is_assigned());
        self.bits = Self::ASSIGNMENT_FLAG_MASK | (value as u8);
    }

    #[inline(always)]
    pub fn unassign(&mut self) {
        debug_assert!(self.is_assigned());
        self.bits &= !Self::ASSIGNMENT_FLAG_MASK;
    }
}

#[derive(Clone)]
struct Decision {
    assignment_order: u32,
}

#[derive(Clone)]
struct Assignment<CompositeExplainKeyT>
where
    CompositeExplainKeyT: Copy,
{
    index: u32,
    decision_level: u32,
    reason: Reason<CompositeExplainKeyT>,
}
