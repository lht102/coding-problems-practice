struct NumArray {
    psum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            psum: std::iter::once(0)
                .chain(nums.into_iter().scan(0, |acc, num| {
                    *acc += num;
                    Some(*acc)
                }))
                .collect(),
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.psum[right as usize + 1] - self.psum[left as usize]
    }
}
