use crate::hydro::value::Value;

#[derive(Debug)]
pub struct PossibleValue {
    pub ranges: Vec<(bool, Value, Value, bool)>
}

impl PossibleValue {
    pub fn range_inc_exc(from: Value, to: Value) -> Self {
        Self {
            ranges: vec![(true, from, to, false)]
        }
    }

    pub fn range_exc_inc(from: Value, to: Value) -> Self {
        Self {
            ranges: vec![(false, from, to, true)]
        }
    }

    pub fn range_inc_inc(from: Value, to: Value) -> Self {
        Self {
            ranges: vec![(true, from, to, true)]
        }
    }

    pub fn range_exc_exc(from: Value, to: Value) -> Self {
        Self {
            ranges: vec![(false, from, to, false)]
        }
    }

    pub fn intersect(left: Self, right: Self) -> Self {
        let mut results = Vec::new();
        for target in left.ranges {
            let mut ints = Self::intersect_internal(target, right.ranges.clone());
            results.append(&mut ints)
        }
        Self {
            ranges: Self::union_internal(results),
        }
    }

    fn intersect_internal(target: (bool, Value, Value, bool), remaining: Vec<(bool, Value, Value, bool)>) -> Vec<(bool, Value, Value, bool)> {
        let mut overlapping = Vec::new();
        let mut nonoverlapping = Vec::new();
        for range in remaining {
            if Self::overlaps(target.clone(), range.clone()) {
                overlapping.push(range);
            } else {
                nonoverlapping.push(range.clone());
            }
        }

        let mut results = Vec::new();
        for range in &overlapping {
            results.push(Self::intersect_range(target.clone(), range.clone()));
        }

        Self::union_internal(results)
    }

    pub fn intersect_range(a: (bool, Value, Value, bool), b: (bool, Value, Value, bool)) -> (bool, Value, Value, bool) {
        // assume these overlap
        let (mut a_min_included, mut a_min, mut a_max, mut a_max_included) = a;
        let (b_min_included, b_min, b_max, b_max_included) = b;
        if a_min < b_min {
            a_min_included = b_min_included;
            a_min = b_min;
        } else if b_min == a_min {
            a_min_included &= b_min_included;
        }

        if b_max < a_max {
            a_max_included = b_max_included;
            a_min = b_max
        } else if b_max == a_min {
            a_max_included &= b_max_included;
        }
        (a_min_included, a_min, a_max,  a_max_included)

    }

    fn inside(range: (bool, Value, Value, bool), value: Value) -> bool {
        let ( min_included, min, max, max_included ) = range;
        let left = if min_included { min <= value } else { min < value };
        let right = if max_included { max >= value } else { max > value };
        left && right
    }

    fn overlaps(left: (bool, Value, Value, bool), right: (bool, Value, Value, bool)) -> bool {
        let ( min_a_included, min_a, max_a, max_a_included ) = left;
        let ( min_b_included, min_b, max_b, max_b_included ) = right;

        ((max_b > min_a && max_b < max_a) || (min_b > min_a && min_b < max_a))
            || (min_a == max_b && min_a_included && max_b_included)
            || (max_a == min_b && max_a_included && min_b_included)
    }

    pub fn union(left: Self, right: Self) -> Self {
        let mut results = left.ranges;
        let mut right_range = right.ranges;
        results.append(&mut right_range);
        Self {
            ranges: Self::union_internal(results),
        }
    }

    fn union_internal(value: Vec<(bool, Value, Value, bool)>) -> Vec<(bool, Value, Value, bool)> {
        let mut source = value;
        while source.len() != 0 {
            let target = source.first().unwrap();
            let remaining = source.clone();
            let mut overlapping = Vec::new();
            let mut nonoverlapping = Vec::new();
            for range in remaining {
                if Self::overlaps(target.clone(), range.clone()) {
                    overlapping.push(range);
                } else {
                    nonoverlapping.push(range.clone());
                }
            }
            if overlapping.len() == 0 {
                return nonoverlapping;
            }
            let mut a = target.clone();
            for range in overlapping {
                a = Self::union_range(a, range);
            }
            nonoverlapping.push(a);
            source = nonoverlapping;
        }
        Vec::new()
    }

    fn union_range(a: (bool, Value, Value, bool), b: (bool, Value, Value, bool)) -> (bool, Value, Value, bool) {
        let (mut min_included, mut min, mut max, mut max_included) = a;
        let (b_min_included, b_min, b_max, b_max_included) = b;
        if b_min < min {
            min_included = b_min_included;
            min = b_min;
        } else if b_min == min {
            min_included &= b_min_included;
        }

        if b_max > max {
            max_included = b_max_included;
            max = b_max
        } else if b_max == max {
            max_included &= b_max_included;
        }
        (min_included, min, max, max_included)
    }

    pub fn complement(value: Self) -> Self {
        Self {
            ranges: Vec::new()
        }
    }

    pub fn contains(&self, value: Value) -> bool {
        for range in &self.ranges {
            if Self::inside(range.clone(), value.clone()) {
                return true;
            }
        }
        false
    }
}