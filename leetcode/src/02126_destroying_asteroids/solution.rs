struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut arr = asteroids;
        arr.sort_unstable();
        arr.iter().fold(mass as i64, |cur_mass, &num| {
            if cur_mass >= num as i64 {
                return cur_mass + num as i64;
            }
            cur_mass
        }) >= *arr.last().unwrap() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mass = 10;
        let asteroids = vec![3, 9, 19, 5, 21];
        assert!(Solution::asteroids_destroyed(mass, asteroids));

        let mass = 5;
        let asteroids = vec![4, 9, 23, 4];
        assert!(!Solution::asteroids_destroyed(mass, asteroids));
    }
}
