use cdcl_engine::{EngineTrait, Literal, Reason};
use utility::{Array, Map};

use crate::engine::{Constraint, SATEngine};

#[derive(Default)]
pub struct Simplify {
    decision_level_to_min_assignment_level: Map<u32, u32>,
    variable_index_to_redundancy: Map<u32, bool>,
    literal_stack: Array<u32, Literal>,
}

impl Simplify {
    #[inline(never)]
    pub fn simplify(&mut self, clause: &mut Array<u32, Literal>, engine: &SATEngine) {
        if clause.len() <= 2 {
            return;
        }
        // 割当レベルの昇順にソート
        clause.sort_unstable_by_key(|&literal| engine.get_assignment_order(literal.index()));
        //
        self.decision_level_to_min_assignment_level.clear();
        self.variable_index_to_redundancy.clear();
        for &literal in clause.iter() {
            debug_assert!(engine.is_false(literal));
            let decision_level = engine.get_decision_level(literal.index());
            let assignment_order = engine.get_assignment_order(literal.index());
            if self
                .decision_level_to_min_assignment_level
                .get(decision_level)
                .is_none_or(|&a| a > assignment_order)
            {
                self.decision_level_to_min_assignment_level.insert(decision_level, assignment_order);
            }
            self.variable_index_to_redundancy.insert(literal.index(), true);
        }
        //
        for k in (0..clause.len()).rev() {
            let literal = clause[k];
            self.variable_index_to_redundancy.remove(literal.index());
            self.literal_stack.clear();
            if self.is_redundant(literal.index(), engine) {
                clause.swap_remove(k);
            }
            debug_assert!(self.literal_stack.is_empty());
        }
    }

    fn is_redundant(&mut self, index: u32, engine: &SATEngine) -> bool {
        if let Some(is_redundant) = self.variable_index_to_redundancy.get(index) {
            // 当該変数がキャッシュに含まれていればキャッシュの内容を返却
            return *is_redundant;
        }
        let mut is_redundant = true;
        if engine.is_assigned(index) {
            let literal = Literal::new(index, engine.get_value(index));
            let decision_level = engine.get_decision_level(literal.index());
            let assignment_order = engine.get_assignment_order(literal.index());
            let reason = engine.get_reason(literal.index()).unwrap();
            if decision_level == 0 {
                // 決定レベルが 0 ならば true
                is_redundant = true;
            } else if self
                .decision_level_to_min_assignment_level
                .get(decision_level)
                .is_none_or(|&a| assignment_order <= a)
            {
                // 当該変数の割当レベルが decision_level ごとの最小割当レベル以下ならば false
                is_redundant = false;
            } else if reason.is_decision() {
                // 当該変数が決定変数ならば false
                is_redundant = false;
            } else if let Reason::Propagation { explain_key } = reason {
                // 当該変数の割当を説明する節を取得
                let reason_constraint: Constraint<_> = engine.explain(explain_key.into()).into();
                // 現在のスタックサイズを取得
                let n = self.literal_stack.len();
                // 当該変数以外の変数(当該変数への割当の原因になっている変数)をスタックに積む
                self.literal_stack.extend(reason_constraint.iter().filter(|&l| l.index() != index));
                // NOTE: ループを 2 回に分けて，定数数時間でできる判定を先に行ったほうが良い
                // 割当の原因になっている全変数について再帰して判定
                for k in n..self.literal_stack.len() {
                    is_redundant &= self.is_redundant(self.literal_stack[k].index(), engine);
                    if !is_redundant {
                        break;
                    }
                }
                // スタックをもとに戻す
                self.literal_stack.truncate(n);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        // 判定結果をキャッシュ(次回の探索時の枝狩りのため)
        self.variable_index_to_redundancy.insert(index, is_redundant);
        // 判定結果を返却
        is_redundant
    }
}
