struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const M: i64 = 1000000007;
        let mut arr = arr;
        arr.push(0);
        let mut res = 0;
        let mut st = Vec::new();
        for (i, &num) in arr.iter().enumerate() {
            while !st.is_empty() && arr[*st.last().unwrap()] >= num {
                let smallest = st.pop().unwrap();
                let left = if st.is_empty() {
                    smallest + 1
                } else {
                    smallest - *st.last().unwrap()
                };
                let right = i - smallest;
                res = (res + (left as i64 * right as i64 * arr[smallest] as i64) % M) % M;
            }
            st.push(i);
        }
        res as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![3, 1, 2, 4];
        assert_eq!(Solution::sum_subarray_mins(arr), 17);

        let arr = vec![11, 81, 94, 43, 3];
        assert_eq!(Solution::sum_subarray_mins(arr), 444);
    }
}
