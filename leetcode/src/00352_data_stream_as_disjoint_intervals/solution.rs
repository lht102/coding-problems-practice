use std::collections::BTreeMap;

struct SummaryRanges {
    map: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn add_num(&mut self, value: i32) {
        let mut selected_start = None;
        if let Some((&start, end)) = self.map.range_mut(..=value).last() {
            if *end + 1 >= value {
                *end = value.max(*end);
                selected_start = Some(start);
            }
        }
        if selected_start.is_none() {
            self.map.insert(value, value);
            selected_start = Some(value);
        }
        if let Some(end) = self.map.remove(&(value + 1)) {
            *self.map.get_mut(&selected_start.unwrap()).unwrap() = end;
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.map
            .iter()
            .map(|(&k, &v)| vec![k, v])
            .collect::<Vec<_>>()
    }
}
