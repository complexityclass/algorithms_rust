use core::fmt;
use std::cmp::Ordering;

pub fn insertion_sort<T: Ord>(v: &mut [T]) {
    insertion_sort_by(v, |a, b| a.partial_cmp(b).unwrap())
}

pub fn insertion_sort_by<T: Ord, C>(v: &mut [T], compare: C)
where
    C: Fn(&T, &T) -> Ordering,
{
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && compare(&v[j], &v[j - 1]).is_le() {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}
