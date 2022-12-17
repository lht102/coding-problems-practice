struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        tokens.iter().fold(Vec::new(), |mut st, token| {
            if let Ok(num) = token.parse() {
                st.push(num);
            } else if let (Some(rhs), Some(lhs)) = (st.pop(), st.pop()) {
                st.push(match token.as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "*" => lhs * rhs,
                    "/" => lhs / rhs,
                    _ => unreachable!(),
                })
            }
            st
        })[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tokens = vec![
            String::from("2"),
            String::from("1"),
            String::from("+"),
            String::from("3"),
            String::from("*"),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 9);

        let tokens = vec![
            String::from("4"),
            String::from("13"),
            String::from("5"),
            String::from("/"),
            String::from("+"),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 6);

        let tokens = vec![
            String::from("10"),
            String::from("6"),
            String::from("9"),
            String::from("3"),
            String::from("+"),
            String::from("-11"),
            String::from("*"),
            String::from("/"),
            String::from("*"),
            String::from("17"),
            String::from("+"),
            String::from("5"),
            String::from("+"),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
