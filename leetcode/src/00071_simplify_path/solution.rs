struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        format!(
            "/{}",
            path.split('/')
                .filter(|x| !x.is_empty())
                .fold(Vec::new(), |mut st, p| {
                    match p {
                        ".." => {
                            st.pop();
                        }
                        "." => (),
                        p => st.push(p),
                    }
                    st
                })
                .join("/")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = String::from("/home/");
        assert_eq!(Solution::simplify_path(path), String::from("/home"));

        let path = String::from("/../");
        assert_eq!(Solution::simplify_path(path), String::from("/"));

        let path = String::from("/home//foo/");
        assert_eq!(Solution::simplify_path(path), String::from("/home/foo"));
    }
}
