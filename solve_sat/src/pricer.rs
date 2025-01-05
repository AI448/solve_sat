use utility::{Array, HeapedMap};

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
    time_constant: f64,
    activities: Array<u32, f64>,
    unassigned_variable_queue: HeapedMap<u32, f64, CompareValues>,
    activity_increase_value: f64,
}

impl Pricer {
    pub fn new(time_constant: f64) -> Self {
        Self {
            time_constant: time_constant,
            activities: Array::default(),
            unassigned_variable_queue: HeapedMap::default(),
            activity_increase_value: 1.0,
        }
    }

    pub fn add_variable(&mut self, initial_activity: f64, is_assigned: bool) {
        let index = self.activities.len();
        self.activities.push(initial_activity);
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

    pub fn increase_price(&mut self, indices: impl Iterator<Item = u32>) {
        self.activity_increase_value /= 1.0 - 1.0 / self.time_constant;
        if self.activity_increase_value > 1e4 {
            for index in 0..self.activities.len() {
                self.activities[index] /= self.activity_increase_value;
                if self.unassigned_variable_queue.contains_key(index) {
                    self.unassigned_variable_queue.insert(index, self.activities[index]);
                }
            }
            self.activity_increase_value = 1.0;
        }
        for index in indices {
            self.activities[index] += self.activity_increase_value;
            if self.unassigned_variable_queue.contains_key(index) {
                self.unassigned_variable_queue.insert(index, self.activities[index]);
            }
        }
    }
}
