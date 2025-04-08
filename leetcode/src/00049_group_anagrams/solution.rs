use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(HashMap::<[usize; 26], Vec<String>>::new(), |mut map, s| {
                map.entry(Solution::get_hash_code(&s)).or_default().push(s);
                map
            })
            .into_values()
            .collect()
    }

    fn get_hash_code(s: &str) -> [usize; 26] {
        s.bytes().fold([0_usize; 26], |mut arr, ch| {
            arr[(ch - b'a') as usize] += 1;
            arr
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let actual = Solution::group_anagrams(strs);
        assert!([vec![String::from("bat")],
            vec![String::from("tan"), String::from("nat")],
            vec![
                String::from("eat"),
                String::from("tea"),
                String::from("ate")
            ]]
        .iter()
        .all(|item| actual.contains(item)));

        let strs = vec![String::from("")];
        let actual = Solution::group_anagrams(strs);
        assert!([vec![String::from("")]]
            .iter()
            .all(|item| actual.contains(item)));

        let strs = vec![String::from("a")];
        let actual = Solution::group_anagrams(strs);
        assert!([vec![String::from("a")]]
            .iter()
            .all(|item| actual.contains(item)));
    }
}
