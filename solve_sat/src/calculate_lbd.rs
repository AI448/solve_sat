use cdcl_engine::{EngineTrait, Literal};
use std::cell::RefCell;
use utility::Set;

// NOTE: clause_theory の中で実装すればいいかも
#[derive(Default)]
pub struct CalculatePLBD {
    decision_levels: RefCell<Set<u32>>,
}

impl Clone for CalculatePLBD {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl CalculatePLBD {
    pub fn calculate(&self, literals: impl Iterator<Item = Literal>, engine: &impl EngineTrait) -> u32 {
        let mut decision_levels = self.decision_levels.borrow_mut();
        decision_levels.clear();
        let mut number_of_literals = 0;
        let mut number_of_not_falses = 0;
        for literal in literals {
            number_of_literals += 1;
            if engine.is_false(literal) {
                decision_levels.insert(engine.get_decision_level(literal.index()));
            } else {
                number_of_not_falses += 1;
            }
        }
        if number_of_not_falses <= 1 {
            return decision_levels.len();
        } else {
            return number_of_literals - 1;
        }
    }
}
