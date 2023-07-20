struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut st = Vec::new();
        for asteroid in asteroids {
            if asteroid < 0 {
                while let Some(&top) = st.last() {
                    if top > 0 && top + asteroid < 0 {
                        st.pop();
                    } else {
                        break;
                    }
                }
                if let Some(&top) = st.last() {
                    if top > 0 {
                        match top + asteroid {
                            0 => {
                                st.pop();
                                continue;
                            }
                            x if x > 0 => continue,
                            _ => (),
                        }
                    }
                }
            }
            st.push(asteroid);
        }
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let asteroids = vec![5, 10, -5];
        assert_eq!(Solution::asteroid_collision(asteroids), vec![5, 10]);

        let asteroids = vec![8, -8];
        assert_eq!(Solution::asteroid_collision(asteroids), vec![]);

        let asteroids = vec![10, 2, -5];
        assert_eq!(Solution::asteroid_collision(asteroids), vec![10]);

        let asteroids = vec![-2, -1, 1, 2];
        assert_eq!(Solution::asteroid_collision(asteroids), vec![-2, -1, 1, 2]);
    }
}
