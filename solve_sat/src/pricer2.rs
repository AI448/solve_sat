use cdcl_engine::EngineTrait;
use utility::{Array, HeapedMap, Set};

#[derive(Default, Clone, Copy)]
struct CompareValues;

impl<IndexT, ValueT> FnOnce<(&(IndexT, ValueT), &(IndexT, ValueT))> for CompareValues
where
    ValueT: std::cmp::PartialOrd,
{
    type Output = std::cmp::Ordering;
    extern "rust-call" fn call_once(
        self,
        ((_, lhs), (_, rhs)): (&(IndexT, ValueT), &(IndexT, ValueT)),
    ) -> Self::Output {
        rhs.partial_cmp(lhs).unwrap()
    }
}

impl<IndexT, ValueT> FnMut<(&(IndexT, ValueT), &(IndexT, ValueT))> for CompareValues
where
    ValueT: std::cmp::PartialOrd,
{
    extern "rust-call" fn call_mut(
        &mut self,
        ((_, lhs), (_, rhs)): (&(IndexT, ValueT), &(IndexT, ValueT)),
    ) -> Self::Output {
        rhs.partial_cmp(lhs).unwrap()
    }
}

impl<IndexT, ValueT> Fn<(&(IndexT, ValueT), &(IndexT, ValueT))> for CompareValues
where
    ValueT: std::cmp::PartialOrd,
{
    extern "rust-call" fn call(&self, ((_, lhs), (_, rhs)): (&(IndexT, ValueT), &(IndexT, ValueT))) -> Self::Output {
        rhs.partial_cmp(lhs).unwrap()
    }
}

// NOTE: ネーミングが微妙な気もするので，実装が固まったら再検討する
#[derive(Clone)]
pub struct Pricer {
    activities: Array<u32, f64>,
    m: Array<u32, f64>,
    v: Array<u32, f64>,
    time: usize,
    target_variables: Set<u32>,
    unassigned_variable_queue: HeapedMap<u32, f64, CompareValues>,
}

impl Pricer {
    pub fn new(time_constant: f64) -> Self {
        Self {
            activities: Array::default(),
            m: Array::default(),
            v: Array::default(),
            time: 0,
            target_variables: Set::default(),
            unassigned_variable_queue: HeapedMap::default(),
        }
    }

    pub fn add_variable(&mut self, initial_activity: f64, is_assigned: bool) {
        let index = self.activities.len();
        self.activities.push(initial_activity);
        self.m.push(0.0);
        self.v.push(0.0);
        if !is_assigned {
            self.unassigned_variable_queue.insert(index, initial_activity);
        }
    }

    pub fn set_to_unassigned(&mut self, index: u32) {
        if !self.unassigned_variable_queue.contains_key(index) {
            let activity = self.activities[index];
            self.unassigned_variable_queue.insert(index, activity);
        }
    }

    pub fn peek(&self) -> Option<u32> {
        return self.unassigned_variable_queue.first().map(|(&index, _)| index);
    }

    pub fn set_to_assigned(&mut self, index: u32) {
        self.unassigned_variable_queue.remove(index);
    }

    pub fn increase_price(
        &mut self,
        indices: impl Iterator<Item = u32>,
        engine: &impl EngineTrait,
        backjump_level: u32,
    ) {
        self.time += 1;
        self.target_variables.clear();
        for index in indices {
            if engine.get_decision_level(index) == engine.current_decision_level() {
                self.target_variables.insert(index);
            } else {
                self.activities[index] += 0.1 * (1.0 - self.activities[index]);
                if self.unassigned_variable_queue.contains_key(index) {
                    self.unassigned_variable_queue.insert(index, self.activities[index]);
                }
            }
        }
        for decision_level in (backjump_level + 1)..(engine.current_decision_level() + 1) {
            // for decision_level in engine.current_decision_level()..(engine.current_decision_level() + 1) {
            for assignment_level in engine.get_assignment_order_range(decision_level) {
                let index = engine.get_assignment(assignment_level).index();
                let g = (if self.target_variables.contains_key(index) { 1.0 } else { 0.0 }) - self.activities[index];
                self.activities[index] += 0.1 * g;
                if self.unassigned_variable_queue.contains_key(index) {
                    self.unassigned_variable_queue.insert(index, self.activities[index]);
                }
            }
        }
        // for index in indices {
        //     self.target_variables.insert(index);
        // }
        // for decision_level in 1..(engine.current_decision_level() + 1) {
        //     for assignment_level in engine.get_assignment_order_range(decision_level) {
        //         let index = engine.get_assignment(assignment_level).index();
        //         let g = (if self.target_variables.contains_key(index) { 1.0 } else {0.0}) - self.activities[index];
        //         let a = if decision_level <= backjump_level {0.0001} else {0.1};
        //         self.activities[index] += a * g;
        //         if self.unassigned_variable_queue.contains_key(index) {
        //             self.unassigned_variable_queue.insert(index, self.activities[index]);
        //         }
        //     }
        // }
    }
}
