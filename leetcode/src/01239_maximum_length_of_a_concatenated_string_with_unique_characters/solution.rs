struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let unique_bitsets = arr
            .iter()
            .filter_map(|s| {
                let mut bitset: i32 = 0;
                for ch in s.bytes() {
                    bitset |= 1 << (ch - b'a') as i32;
                }
                (bitset.count_ones() == s.len() as _).then_some(bitset)
            })
            .collect::<Vec<_>>();
        let mut concat_bitsets = vec![0];
        for unique_bitset in unique_bitsets {
            concat_bitsets.append(
                &mut concat_bitsets
                    .iter()
                    .filter_map(|bitset| {
                        (bitset & unique_bitset == 0).then_some(bitset | unique_bitset)
                    })
                    .collect(),
            );
        }
        concat_bitsets
            .into_iter()
            .map(i32::count_ones)
            .max()
            .unwrap() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![String::from("un"), String::from("iq"), String::from("ue")];
        assert_eq!(Solution::max_length(arr), 4);

        let arr = vec![
            String::from("cha"),
            String::from("r"),
            String::from("act"),
            String::from("ers"),
        ];
        assert_eq!(Solution::max_length(arr), 6);

        let arr = vec![String::from("abcdefghijklmnopqrstuvwxyz")];
        assert_eq!(Solution::max_length(arr), 26);

        let arr = vec![
            String::from("jnfbyktlrqumowxd"),
            String::from("mvhgcpxnjzrdei"),
        ];
        assert_eq!(Solution::max_length(arr), 16);
    }
}
