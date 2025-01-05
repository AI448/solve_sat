use crate::{array::Array, index::UnsignedInt};
use std::cmp::Ordering::Less;

#[inline(always)]
fn parent_of<SizeT>(position: SizeT) -> SizeT
where
    SizeT: UnsignedInt,
{
    debug_assert!(position != SizeT::ZERO);
    (position + SizeT::from_usize(1)) / SizeT::from_usize(2) - SizeT::from_usize(1)
}

#[inline(always)]
fn left_of<SizeT>(position: SizeT) -> SizeT
where
    SizeT: UnsignedInt,
{
    (position + SizeT::from_usize(1)) * SizeT::from_usize(2) - SizeT::from_usize(1)
}

#[inline(always)]
fn right_of<SizeT>(position: SizeT) -> SizeT
where
    SizeT: UnsignedInt,
{
    (position + SizeT::from_usize(1)) * SizeT::from_usize(2)
}

pub fn up_heap<SizeT: UnsignedInt, ValueT>(
    array: &mut Array<SizeT, ValueT>,
    position: SizeT,
    compare: impl Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
) {
    up_heap_with_callback(array, position, compare, |_, _| ());
}

pub fn down_heap<SizeT: UnsignedInt, ValueT>(
    array: &mut Array<SizeT, ValueT>,
    position: SizeT,
    compare: impl std::ops::Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
) {
    down_heap_with_callback(array, position, compare, |_, _| ());
}

pub fn update_heap<SizeT: UnsignedInt, ValueT>(
    array: &mut Array<SizeT, ValueT>,
    position: SizeT,
    compare: impl std::ops::Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
) {
    update_heap_with_callback(array, position, compare, |_, _| ());
}

pub fn update_heap_with_callback<SizeT: UnsignedInt, ValueT>(
    array: &mut Array<SizeT, ValueT>,
    position: SizeT,
    compare: impl Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
    callback_swap: impl FnMut(&ValueT, &ValueT),
) {
    debug_assert!(position < array.len());
    if position != SizeT::ZERO && compare(&array[position], &array[parent_of(position)]) == Less {
        up_heap_with_callback(array, position, compare, callback_swap);
    } else {
        down_heap_with_callback(array, position, compare, callback_swap);
    }
}

pub fn up_heap_with_callback<SizeT: UnsignedInt, ValueT>(
    array: &mut Array<SizeT, ValueT>,
    position: SizeT,
    compare: impl Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
    mut callback_swap: impl FnMut(&ValueT, &ValueT),
) {
    debug_assert!(position < array.len());
    let mut current = position;
    loop {
        if current == SizeT::ZERO {
            break;
        }
        let parent = parent_of(current);
        if compare(&array[current], &array[parent]) == Less {
            array.swap(parent, current);
            callback_swap(&array[parent], &array[current]);
            current = parent;
        } else {
            break;
        }
    }
}

pub fn down_heap_with_callback<SizeT: UnsignedInt, ValueT>(
    array: &mut Array<SizeT, ValueT>,
    position: SizeT,
    compare: impl std::ops::Fn(&ValueT, &ValueT) -> std::cmp::Ordering,
    mut callback_swap: impl FnMut(&ValueT, &ValueT),
) {
    debug_assert!(position < array.len());
    let mut current = position;
    loop {
        let left = left_of(current);
        if left >= array.len() {
            break;
        }
        let right = right_of(current);
        let child = if right >= array.len() || compare(&array[left], &array[right]) == Less { left } else { right };
        if compare(&array[child], &array[current]) == Less {
            array.swap(current, child);
            callback_swap(&array[child], &array[current]);
            current = child;
        } else {
            break;
        }
    }
}
