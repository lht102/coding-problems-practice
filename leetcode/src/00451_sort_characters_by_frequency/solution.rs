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
                (cnt > 0).then_some(std::iter::repeat(char::from_u32(ch as u32).unwrap()).take(cnt))
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
        let s = String::from("tree");
        assert_eq!(Solution::frequency_sort(s), String::from("eert"));

        let s = String::from("cccaaa");
        assert_eq!(Solution::frequency_sort(s), String::from("aaaccc"));

        let s = String::from("Aabb");
        assert_eq!(Solution::frequency_sort(s), String::from("bbAa"));
    }
}
