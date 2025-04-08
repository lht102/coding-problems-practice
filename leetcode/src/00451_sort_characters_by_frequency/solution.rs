struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let freq = s.chars().fold([0usize; 128], |mut arr, ch| {
            arr[ch as usize] += 1;
            arr
        });
        let mut arr = freq.into_iter().enumerate().collect::<Vec<_>>();
        arr.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        arr.into_iter()
            .filter_map(|(ch, cnt)| {
                (cnt > 0).then_some(std::iter::repeat_n(char::from_u32(ch as u32).unwrap(), cnt))
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Solution::frequency_sort("tree".to_string());
        assert!(matches!(&*s, "eert" | "eetr"));

        let s = Solution::frequency_sort("cccaaa".to_string());
        assert!(matches!(&*s, "aaaccc" | "cccaaa"));

        let s = Solution::frequency_sort("Aabb".to_string());
        assert!(matches!(&*s, "bbAa" | "bbaA"));
    }
}
