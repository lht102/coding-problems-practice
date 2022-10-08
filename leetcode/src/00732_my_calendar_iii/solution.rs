use std::collections::BTreeMap;

struct Solution;

struct MyCalendarThree {
    map: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.map.entry(start).or_default() += 1;
        *self.map.entry(end).or_default() -= 1;
        let mut res = 0;
        let mut cur = 0;
        for val in self.map.values() {
            cur += val;
            res = res.max(cur);
        }
        res
    }
}
