struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut res = 0;
        let (mut i, mut j) = (0, people.len() - 1);
        while i <= j {
            res += 1;
            if people[i] + people[j] <= limit {
                i += 1;
            }
            if j == 0 {
                break;
            } else {
                j -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let people = vec![1, 2];
        let limit = 3;
        assert_eq!(Solution::num_rescue_boats(people, limit), 1);

        let people = vec![3, 2, 2, 1];
        let limit = 3;
        assert_eq!(Solution::num_rescue_boats(people, limit), 3);

        let people = vec![3, 5, 3, 4];
        let limit = 5;
        assert_eq!(Solution::num_rescue_boats(people, limit), 4);
    }
}
