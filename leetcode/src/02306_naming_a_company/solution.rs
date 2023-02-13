use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let get_index = |x: u8| (x - b'a') as usize;
        let suffixes_by_first_letter =
            ideas
                .iter()
                .fold(vec![HashSet::<&[u8]>::new(); 26], |mut acc, idea| {
                    let idea = idea.as_bytes();
                    acc[get_index(idea[0])].insert(&idea[1..]);
                    acc
                });
        let mut res = 0;
        for i in 0..25 {
            for j in i + 1..26 {
                let mut num_of_overlaps = 0;
                for suffix in &suffixes_by_first_letter[i] {
                    if suffixes_by_first_letter[j].contains(suffix) {
                        num_of_overlaps += 1;
                    }
                }
                res += 2
                    * (suffixes_by_first_letter[i].len() - num_of_overlaps)
                    * (suffixes_by_first_letter[j].len() - num_of_overlaps);
            }
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ideas = vec![
            String::from("coffee"),
            String::from("donuts"),
            String::from("time"),
            String::from("toffee"),
        ];
        assert_eq!(Solution::distinct_names(ideas), 6);

        let ideas = vec![String::from("lack"), String::from("back")];
        assert_eq!(Solution::distinct_names(ideas), 0);
    }
}
