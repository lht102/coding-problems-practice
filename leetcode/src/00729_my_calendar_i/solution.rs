use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendar {
    calendar: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self
            .calendar
            .range(start..)
            .next()
            .map_or(false, |(&s, _)| end > s)
            || self
                .calendar
                .range(..start)
                .rev()
                .next()
                .map_or(false, |(_, &e)| e > start)
        {
            return false;
        }
        self.calendar.insert(start, end);
        true
    }
}
