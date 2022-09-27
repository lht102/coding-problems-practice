struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.chars().collect::<Vec<_>>();
        let mut left = 0;
        for right in 0..dominoes.len() {
            if dominoes[right] == '.' {
                continue;
            } else if dominoes[left] == dominoes[right]
                || dominoes[left] == '.' && dominoes[right] == 'L'
            {
                for i in left..right {
                    dominoes[i] = dominoes[right];
                }
            } else if dominoes[left] == 'R' && dominoes[right] == 'L' {
                for i in 1..=(right - left - 1) / 2 {
                    dominoes[left + i] = 'R';
                    dominoes[right - i] = 'L';
                }
            }
            left = right;
        }
        if dominoes[left] == 'R' {
            for ch in dominoes.iter_mut().skip(left + 1) {
                *ch = 'R';
            }
        }
        dominoes.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let dominoes = String::from("RR.L");
        assert_eq!(Solution::push_dominoes(dominoes), String::from("RR.L"));

        let dominoes = String::from(".L.R...LR..L..");
        assert_eq!(
            Solution::push_dominoes(dominoes),
            String::from("LL.RR.LLRRLL..")
        );
    }
}
