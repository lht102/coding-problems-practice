struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let discount = discount as f64;
        let words = sentence.split_whitespace().collect::<Vec<_>>();
        let mut res = Vec::new();
        for w in words {
            if let Some(stripped) = w.strip_prefix('$') {
                if let Ok(num) = stripped.parse::<f64>() {
                    let p = num * (100.0 - discount) / 100.0;
                    res.push(format!("${p:.2}"));
                    continue;
                }
            }
            res.push(w.to_string());
        }
        res.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sentence = String::from("there are $1 $2 and 5$ candies in the shop");
        let discount = 50;
        assert_eq!(
            Solution::discount_prices(sentence, discount),
            String::from("there are $0.50 $1.00 and 5$ candies in the shop")
        );

        let sentence = String::from("1 2 $3 4 $5 $6 7 8$ $9 $10$");
        let discount = 100;
        assert_eq!(
            Solution::discount_prices(sentence, discount),
            String::from("1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$")
        );
    }
}
