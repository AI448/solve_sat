use cdcl_engine::{Boolean, EngineTrait, Literal, Reason};
use utility::{Array, Map, Set};

use crate::{
    clause_theory::ClauseTrait,
    engine::{Constraint, ExplainKey, SATEngine, make_constraint},
    simplify::Simplify,
};

pub enum AnalyzeResult<ClauseT: ClauseTrait, IteratorT: Iterator<Item = u32> + Clone> {
    Unsatisfiable,
    Backjumpable { backjump_decision_level: u32, learnt_clause: Constraint<ClauseT>, related_variables: IteratorT },
}

#[derive(Default)]
pub struct Analyze {
    simplify: Simplify,
    learnt_clause: Map<u32, Boolean>,
    related_variables: Set<u32>,
    buffer: Array<u32, Literal>,
}

impl Analyze {
    pub fn analyze<'a>(
        &'a mut self,
        explain_key: ExplainKey,
        engine: &SATEngine,
    ) -> AnalyzeResult<impl ClauseTrait + 'a, impl Iterator<Item = u32> + Clone + 'a> {
        self.learnt_clause.clear();
        self.related_variables.clear();
        {
            let conflicting_constraint = engine.explain(explain_key.into()).into();
            // eprintln!("CONFLICTING {}", &conflicting_constraint);
            Self::resolve(&mut self.learnt_clause, conflicting_constraint, engine);
        }
        loop {
            if self.learnt_clause.is_empty() {
                // 長さ 0 の節が現れたら充足不可能
                return AnalyzeResult::Unsatisfiable;
            }

            if let Some(backjump_decision_level) = Self::calculate_backjump_decision_level(&self.learnt_clause, engine)
            {
                // バックジャンプ可能な節が得られた
                self.buffer.clear();
                self.buffer.extend(self.learnt_clause.iter().map(|(&index, &value)| Literal::new(index, value)));
                self.simplify.simplify(&mut self.buffer, engine);

                for literal in self.buffer.iter() {
                    self.related_variables.insert(literal.index());
                }
                return AnalyzeResult::Backjumpable {
                    backjump_decision_level: backjump_decision_level,
                    learnt_clause: make_constraint(self.buffer.iter().cloned()),
                    related_variables: self.related_variables.iter().cloned(),
                };
            }

            // 最後に割り当てられたリテラルを特定
            let last_assigned_literal = Self::find_last_assigned_literal(&self.learnt_clause, engine);
            // 割り当て理由を取得
            let reason = engine.get_reason(last_assigned_literal.index()).unwrap();
            let Reason::Propagation { explain_key } = reason else {
                // 割り当て理由は伝播であるはず
                unreachable!()
            };
            let reason_constraint = engine.explain(explain_key.into()).into();
            // eprintln!("REASON_CONSTRAINT: {}", &reason_constraint);

            // 節融合
            Self::resolve(&mut self.learnt_clause, reason_constraint, engine);
            debug_assert!(!self.learnt_clause.contains_key(last_assigned_literal.index()));

            self.related_variables.insert(last_assigned_literal.index());
        }
    }

    fn calculate_backjump_decision_level(learnt_clause: &Map<u32, Boolean>, engine: &SATEngine) -> Option<u32> {
        let mut top2_decision_levels = [0; 2];
        for (&index, &value) in learnt_clause.iter() {
            let literal = Literal::new(index, value);
            debug_assert!(engine.is_false(literal));
            let decision_level = engine.get_decision_level(literal.index());
            if decision_level > top2_decision_levels[0] {
                top2_decision_levels[1] = top2_decision_levels[0];
                top2_decision_levels[0] = decision_level;
            } else if decision_level > top2_decision_levels[1] {
                top2_decision_levels[1] = decision_level;
            }
        }
        if top2_decision_levels[0] > top2_decision_levels[1] {
            return Some(top2_decision_levels[1]);
        } else {
            return None;
        }
    }

    fn find_last_assigned_literal(learnt_clause: &Map<u32, Boolean>, engine: &SATEngine) -> Literal {
        let mut max_assignment_order = None;
        let mut last_assigned_literal = None;
        for (&index, &value) in learnt_clause.iter() {
            let literal = Literal::new(index, value);
            debug_assert!(engine.is_false(literal));
            let assignment_order = engine.get_assignment_order(literal.index());
            if max_assignment_order.is_none_or(|x| assignment_order > x) {
                max_assignment_order = Some(assignment_order);
                last_assigned_literal = Some(!literal);
            }
        }
        return last_assigned_literal.unwrap();
    }

    fn resolve(learnt_clause: &mut Map<u32, Boolean>, constraint: Constraint<impl ClauseTrait>, engine: &SATEngine) {
        match constraint {
            Constraint::MonadicClause(literal) => {
                Self::resolve_by_iterator(learnt_clause, [literal].into_iter(), engine)
            }
            Constraint::BinaryClause(binary_clause) => {
                Self::resolve_by_iterator(learnt_clause, binary_clause.iter(), engine)
            }
            Constraint::Clause(clause) => Self::resolve_by_iterator(learnt_clause, clause.iter_literals(), engine),
        };
    }

    fn resolve_by_iterator(
        learnt_clause: &mut Map<u32, Boolean>,
        literals: impl Iterator<Item = Literal>,
        engine: &SATEngine,
    ) {
        for literal in literals {
            if engine.get_decision_level(literal.index()) == 0 {
                continue;
            }
            if learnt_clause.contains_key(literal.index()) {
                if *learnt_clause.get(literal.index()).unwrap() != literal.value() {
                    learnt_clause.remove(literal.index());
                }
            } else {
                learnt_clause.insert(literal.index(), literal.value());
            }
        }
    }
}
