use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut pq = BinaryHeap::new();
        let mut cur_time = 0;
        for c in &courses {
            if cur_time + c[0] <= c[1] {
                pq.push(c[0]);
                cur_time += c[0];
            } else if let Some(&top) = pq.peek() {
                if top > c[0] {
                    cur_time += c[0] - top;
                    pq.pop();
                    pq.push(c[0]);
                }
            }
        }
        pq.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let courses = vec![vec![1, 2]];
        assert_eq!(Solution::schedule_course(courses), 1);

        let courses = vec![vec![3, 2], vec![4, 3]];
        assert_eq!(Solution::schedule_course(courses), 0);
    }
}
