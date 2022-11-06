struct Solution;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        if k == 1 {
            let mut smallest = s.clone();
            for _ in 1..s.len() {
                s.rotate_left(1);
                if s < smallest {
                    smallest = s.clone();
                }
            }
            smallest.into_iter().collect()
        } else {
            s.sort_unstable();
            s.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("cba");
        let k = 1;
        assert_eq!(Solution::orderly_queue(s, k), String::from("acb"));

        let s = String::from("baaca");
        let k = 3;
        assert_eq!(Solution::orderly_queue(s, k), String::from("aaabc"));
    }
}
