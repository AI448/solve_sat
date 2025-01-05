use cdcl_engine::{Literal, PropagationResult, Reason, TheoryAddConstraintTrait, TheoryTrait};
use utility::Array;

#[derive(Clone, Copy)]
pub struct BinaryClause {
    literals: [Literal; 2],
}

impl BinaryClause {
    pub fn new(l0: Literal, l1: Literal) -> Self {
        Self { literals: [l0, l1] }
    }

    pub fn from_iter(mut iterator: impl Iterator<Item = Literal>) -> Self {
        let l0 = iterator.next().unwrap();
        let l1 = iterator.next().unwrap();
        assert!(iterator.next().is_none());
        return Self::new(l0, l1);
    }

    pub fn iter(&self) -> impl Iterator<Item = Literal> + Clone {
        self.literals.iter().cloned()
    }
}

impl From<BinaryClause> for [Literal; 2] {
    fn from(binary_clause: BinaryClause) -> Self {
        return binary_clause.literals;
    }
}

#[derive(Clone, Copy)]
pub struct BinaryClauseTheoryExplainKey {
    pub binary_clause: BinaryClause,
}

#[derive(Default, Clone)]
pub struct BinaryClauseTheorySummary {
    pub number_of_binary_clauses: u32,
    pub number_of_learnt_binary_clauses: u32,
}

#[derive(Default, Clone)]
pub struct BinaryClauseTheory {
    implications: Array<u32, [Array<u32, Literal>; 2]>,
    summary: BinaryClauseTheorySummary,
}

impl TheoryTrait for BinaryClauseTheory {
    type ExplainKey = BinaryClauseTheoryExplainKey;
    type ExplanationConstraint<'a> = BinaryClause;
    type Summary = BinaryClauseTheorySummary;
    fn add_variable(&mut self) {
        self.implications.push([Array::default(), Array::default()]);
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
        debug_assert!(engine.is_true(assigned_literal));
        debug_assert!(engine.get_decision_level(assigned_literal.index()) == engine.current_decision_level());
        for &literal in self.implications[assigned_literal.index()][assigned_literal.value()].iter() {
            let explain_key = Self::ExplainKey { binary_clause: BinaryClause::new(!assigned_literal, literal) };
            if !engine.is_assigned(literal.index()) {
                // literal が未割り当てであれば literal に真を割り当て
                let inner_result = engine.assign(literal, Reason::Propagation { explain_key: explain_key.into() });
                if inner_result.is_conflict() {
                    return inner_result;
                }
            } else if engine.is_false(literal) {
                // literal に false が割当たっていれば矛盾
                return PropagationResult::Conflict { explain_key: explain_key.into() };
            }
        }
        return PropagationResult::Noconflict;
    }

    #[inline(always)]
    fn explain(&self, explain_key: BinaryClauseTheoryExplainKey) -> BinaryClause {
        return explain_key.binary_clause;
    }

    #[inline(always)]
    fn unassign(&mut self, _unassigned_literals: impl Iterator<Item = Literal>) {
        // なにもしない
    }

    #[inline(always)]
    fn reduce_constraints(&mut self) {
        // なにもしない
    }

    fn summary(&self) -> Self::Summary {
        return self.summary.clone();
    }
}

impl TheoryAddConstraintTrait<BinaryClause> for BinaryClauseTheory {
    fn add_constraint<EngineT: cdcl_engine::EngineTrait>(
        &mut self,
        binary_clause: BinaryClause,
        is_learnt: bool,
        engine: &mut EngineT,
    ) -> PropagationResult<EngineT::CompositeExplainKey>
    where
        EngineT::CompositeExplainKey: From<Self::ExplainKey>,
    {
        let [l0, l1]: [Literal; 2] = binary_clause.into();
        debug_assert!(l0.index() != l1.index());
        debug_assert!(!engine.is_false(l0) || !engine.is_false(l1));

        // 既に存在する制約との重複を確認
        if self.implications[l0.index()][!l0.value()].contains(&l1) {
            debug_assert!(self.implications[l1.index()][!l1.value()].contains(&l0));
            return PropagationResult::Noconflict;
        }

        // summary を更新
        self.summary.number_of_binary_clauses += 1;
        if is_learnt {
            self.summary.number_of_learnt_binary_clauses += 1;
        }

        // 制約を追加
        self.implications[l0.index()][!l0.value()].push(l1);
        self.implications[l1.index()][!l1.value()].push(l0);

        let explain_key = Self::ExplainKey { binary_clause: binary_clause };
        // 伝播の発生を確認
        if engine.is_false(l0) && !engine.is_assigned(l1.index()) {
            // l0 が FALSE， l1 が未割り当てであれば l1 を真に
            debug_assert!(engine.get_decision_level(l0.index()) == engine.current_decision_level());
            return engine.assign(l1, Reason::Propagation { explain_key: explain_key.into() });
        } else if !engine.is_assigned(l0.index()) && engine.is_false(l1) {
            // l1 が未割り当て， l1 が FALSE であれば l0 を真に
            debug_assert!(engine.get_decision_level(l1.index()) == engine.current_decision_level());
            return engine.assign(l0, Reason::Propagation { explain_key: explain_key.into() });
        } else {
            // それ以外
            return PropagationResult::Noconflict;
        }
    }
}
