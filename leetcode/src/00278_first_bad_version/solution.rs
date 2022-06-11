struct Solution {
    versions: Vec<bool>,
}

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.isBadVersion(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    pub fn new(n: i32, bad: i32) -> Self {
        let mut versions = vec![false; n as usize];
        for i in bad as usize - 1..n as usize {
            versions[i] = true;
        }
        Self { versions }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        self.versions[version as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 5;
        let bad = 4;
        let s = Solution::new(n, bad);
        assert_eq!(s.first_bad_version(n), bad);

        let n = 1;
        let bad = 1;
        let s = Solution::new(n, bad);
        assert_eq!(s.first_bad_version(n), bad);
    }
}
