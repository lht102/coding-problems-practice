use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut q = VecDeque::from([start]);
        let mut bank: HashSet<String> = HashSet::from_iter(bank.into_iter());
        let mut dist = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(cu) = q.pop_front() {
                    if cu == end {
                        return dist;
                    }
                    for i in 0..8 {
                        for ch in &['A', 'C', 'G', 'T'] {
                            let mutation = format!("{}{}{}", &cu[0..i], ch, &cu[i + 1..]);
                            if bank.remove(&mutation) {
                                q.push_back(mutation);
                            }
                        }
                    }
                }
            }
            dist += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let start = String::from("AACCGGTT");
        let end = String::from("AACCGGTA");
        let bank = vec![String::from("AACCGGTA")];
        assert_eq!(Solution::min_mutation(start, end, bank), 1);

        let start = String::from("AACCGGTT");
        let end = String::from("AAACGGTA");
        let bank = vec![
            String::from("AACCGGTA"),
            String::from("AACCGCTA"),
            String::from("AAACGGTA"),
        ];
        assert_eq!(Solution::min_mutation(start, end, bank), 2);

        let start = String::from("AAAAACCC");
        let end = String::from("AACCCCCC");
        let bank = vec![
            String::from("AAAACCCC"),
            String::from("AAACCCCC"),
            String::from("AACCCCCC"),
        ];
        assert_eq!(Solution::min_mutation(start, end, bank), 3);
    }
}
