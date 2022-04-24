use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    id_to_in: HashMap<i32, (String, i32)>,
    route_to_sum_cnt: HashMap<(String, String), (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.id_to_in.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(v) = self.id_to_in.get(&id) {
            let e = self
                .route_to_sum_cnt
                .entry((v.0.clone(), station_name))
                .or_default();
            e.0 += t - v.1;
            e.1 += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(v) = self.route_to_sum_cnt.get(&(start_station, end_station)) {
            return (v.0 as f64 / v.1 as f64 * 100000.0).round() / 100000.0;
        }
        unreachable!()
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut system = UndergroundSystem::new();
        system.check_in(45, String::from("Leyton"), 3);
        system.check_in(32, String::from("Paradise"), 8);
        system.check_in(27, String::from("Leyton"), 10);
        system.check_out(45, String::from("Waterloo"), 15);
        system.check_out(27, String::from("Waterloo"), 20);
        system.check_out(32, String::from("Cambridge"), 22);
        assert_eq!(
            system.get_average_time(String::from("Paradise"), String::from("Cambridge")),
            14.0
        );
        assert_eq!(
            system.get_average_time(String::from("Leyton"), String::from("Waterloo")),
            11.0
        );
        system.check_in(10, String::from("Leyton"), 24);
        system.check_out(10, String::from("Waterloo"), 38);
        assert_eq!(
            system.get_average_time(String::from("Leyton"), String::from("Waterloo")),
            12.0
        );

        let mut system = UndergroundSystem::new();
        system.check_in(10, String::from("Leyton"), 3);
        system.check_out(10, String::from("Paradise"), 8);
        assert_eq!(
            system.get_average_time(String::from("Leyton"), String::from("Paradise")),
            5.0
        );
        system.check_in(5, String::from("Leyton"), 10);
        system.check_out(5, String::from("Paradise"), 16);
        assert_eq!(
            system.get_average_time(String::from("Leyton"), String::from("Paradise")),
            5.5
        );
        system.check_in(2, String::from("Leyton"), 21);
        system.check_out(2, String::from("Paradise"), 30);
        assert_eq!(
            system.get_average_time(String::from("Leyton"), String::from("Paradise")),
            6.66667
        );
    }
}
