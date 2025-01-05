use cdcl_engine::{
    EngineTrait, Literal, LiteralArray, PropagationResult, Reason, TheoryAddConstraintTrait, TheoryTrait,
};
use utility::Array;

use super::calculate_lbd::CalculatePLBD;

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
pub struct ClauseExplainKey {
    row_id: u32,
}

#[derive(Default, Clone)]
pub struct ClauseTheorySummary {
    pub number_of_clauses: u32,
    pub number_of_learnt_clauses: u32,
}

#[derive(Clone)]
struct Row {
    literals: Array<u32, Literal>,
    is_learnt: bool,
    deleted: bool, // TODO 削除したスロットを再利用できるデータ構造はいずれ考える
    generated_time_stamp: usize,
    plbd: u32,
    activity: f64,
}

#[derive(Clone, Copy)]
struct Watch {
    row_id: u32,
    position: u32, // NOTE: 他に詰め込むものがあれば u8 にすることも検討
}

#[derive(Clone)]
pub struct ClauseTheory {
    calculate_plbd: CalculatePLBD,
    activity_time_constant: f64,
    rows: Array<u32, Row>,
    watches: LiteralArray<Array<u32, Watch>>,
    time: usize,
    activity_increase_value: f64,
    last_reduction_time_stamp: usize,
    summary: ClauseTheorySummary,
}

impl ClauseTheory {
    pub fn new(activity_time_constant: f64) -> Self {
        Self {
            calculate_plbd: CalculatePLBD::default(),
            activity_time_constant: activity_time_constant,
            rows: Array::default(),
            watches: LiteralArray::default(),
            time: 0,
            activity_increase_value: 1.0,
            last_reduction_time_stamp: 0,
            summary: ClauseTheorySummary::default(),
        }
    }
}

impl TheoryTrait for ClauseTheory {
    type ExplainKey = ClauseExplainKey;
    type ExplanationConstraint<'a> = impl ClauseTrait + 'a;
    type Summary = ClauseTheorySummary;

    fn add_variable(&mut self) {
        self.watches.push([Array::default(), Array::default()]);
    }

    fn assign<EngineT>(
        &mut self,
        assigned_literal: Literal,
        engine: &mut EngineT,
    ) -> cdcl_engine::PropagationResult<EngineT::CompositeExplainKey>
    where
        EngineT: cdcl_engine::EngineTrait,
        EngineT::CompositeExplainKey: From<Self::ExplainKey>,
    {
        'loop_for_watches: for k in (0..self.watches[assigned_literal].len()).rev() {
            let watch = self.watches[assigned_literal][k];
            debug_assert!(watch.position == 0 || watch.position == 1);
            let row = &mut self.rows[watch.row_id];
            if row.deleted {
                // 節が削除されていれば監視も削除する
                self.watches[assigned_literal].swap_remove(k);
                continue;
            }
            let watched_literal = row.literals[watch.position];
            debug_assert!(watched_literal == !assigned_literal);
            let another_watched_literal = row.literals[1 - watch.position];
            if engine.is_true(another_watched_literal) {
                // もう一方の監視対象のリテラルが真であれば何もしない
            } else {
                // 監視対象ではないリテラルを走査
                for position in 2..row.literals.len() {
                    let literal = row.literals[position];
                    if !engine.is_false(literal) {
                        // 偽ではないリテラルを発見した場合
                        // 発見したリテラルを監視位置に移動
                        row.literals.swap(watch.position, position);
                        // もとのリテラルの監視を解除
                        self.watches[assigned_literal].swap_remove(k);
                        // 発見したリテラルの否定を監視
                        self.watches[!literal].push(watch);
                        // 次の節へ
                        continue 'loop_for_watches;
                    }
                }
                // 偽ではないリテラルが存在しない場合
                if !engine.is_assigned(another_watched_literal.index()) {
                    // もう一方の監視リテラルが未割り当てである場合には伝播が発生
                    // lbd を更新
                    // if row.plbd > 1 {
                    //     let plbd = self
                    //         .calculate_plbd
                    //         .calculate_clause_plbd(&ClauseView::new(row.literals.iter().cloned()), engine);
                    //     debug_assert!(plbd >= 1);
                    //     row.plbd = u32::min(row.plbd, plbd);
                    // }
                    row.activity += self.activity_increase_value;
                    // もう一方の監視リテラルに真を割り当て
                    let inner_result = engine.assign(another_watched_literal, Reason::Propagation {
                        explain_key: ClauseExplainKey { row_id: watch.row_id }.into(),
                    });
                    if inner_result.is_conflict() {
                        return inner_result;
                    }
                } else {
                    // もう一方の監視リテラルが偽であれば矛盾
                    debug_assert!(engine.is_false(another_watched_literal));
                    return PropagationResult::Conflict {
                        explain_key: ClauseExplainKey { row_id: watch.row_id }.into(),
                    };
                }
            }
        }
        return PropagationResult::Noconflict;
    }

    fn explain(&self, explain_key: ClauseExplainKey) -> Self::ExplanationConstraint<'_> {
        return ClauseView::new(self.rows[explain_key.row_id].literals.iter().cloned());
    }

    fn unassign(&mut self, _unassigned_literals: impl Iterator<Item = Literal>) {
        self.activity_increase_value /= 1.0 - 1.0 / self.activity_time_constant;
        self.time += 1;
    }

    fn reduce_constraints(&mut self) {
        let activity_threshold = f64::powf(
            1.0 - 1.0 / self.activity_time_constant,
            f64::max(self.activity_time_constant, self.time as f64 / 10.0),
        );
        for row in self.rows.iter_mut() {
            row.activity /= self.activity_increase_value;
            if row.is_learnt && !row.deleted && row.activity <= activity_threshold {
                row.deleted = true;
                self.summary.number_of_clauses -= 1;
                self.summary.number_of_learnt_clauses -= 1;
            }
        }
        self.activity_increase_value = 1.0;
    }

    fn summary(&self) -> Self::Summary {
        return self.summary.clone();
    }
}

impl<ClauseT> TheoryAddConstraintTrait<ClauseT> for ClauseTheory
where
    ClauseT: ClauseTrait,
{
    fn add_constraint<EngineT: EngineTrait>(
        &mut self,
        clause: ClauseT,
        is_learnt: bool,
        engine: &mut EngineT,
    ) -> PropagationResult<EngineT::CompositeExplainKey>
    where
        EngineT::CompositeExplainKey: From<Self::ExplainKey>,
    {
        debug_assert!(clause.len() >= 2);

        let mut literals = Array::from_iter(clause.iter_literals());
        literals.sort_unstable_by_key(|&l| {
            if engine.is_true(l) {
                (0, engine.get_assignment_order(l.index()))
            } else if !engine.is_assigned(l.index()) {
                (1, 0)
            } else {
                (2, u32::MAX - engine.get_assignment_order(l.index()))
            }
        });
        debug_assert!(!engine.is_false(literals[0]));

        self.summary.number_of_clauses += 1;
        if is_learnt {
            self.summary.number_of_learnt_clauses += 1;
        }

        let plbd = self.calculate_plbd.calculate(literals.iter().cloned(), engine);

        let row_id = self.rows.len();

        // 先頭 2 つのリテラルを監視対象に
        for watch_position in [0, 1] {
            self.watches[!literals[watch_position]].push(Watch { row_id: row_id, position: watch_position });
        }
        self.rows.push(Row {
            literals: literals,
            is_learnt: is_learnt,
            plbd: plbd,
            activity: self.activity_increase_value,
            deleted: false,
            generated_time_stamp: self.time,
        });
        let row = self.rows.last().unwrap();

        if !engine.is_assigned(row.literals[0].index()) && engine.is_false(row.literals[1]) {
            debug_assert!(engine.get_decision_level(row.literals[1].index()) == engine.current_decision_level());
            return engine.assign(row.literals[0], Reason::Propagation {
                explain_key: ClauseExplainKey { row_id: row_id }.into(),
            });
        } else {
            return PropagationResult::Noconflict;
        }
    }
}
