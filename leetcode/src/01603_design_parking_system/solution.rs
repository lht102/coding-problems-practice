struct Solution;

struct ParkingSystem {
    remainings: [i32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            remainings: [big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let idx = (car_type - 1) as usize;
        if self.remainings[idx] > 0 {
            self.remainings[idx] -= 1;
            return true;
        }
        false
    }
}
