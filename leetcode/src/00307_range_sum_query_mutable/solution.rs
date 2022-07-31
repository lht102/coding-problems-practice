struct Solution;

struct NumArray {
    tree: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums = nums;
        nums.insert(0, 0);
        for i in 1..nums.len() {
            let p = i + Self::lsb(i);
            if p < nums.len() {
                nums[p] += nums[i];
            }
        }
        Self { tree: nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.add(index as _, val - self.sum_range(index, index))
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sum(right as _) - self.prefix_sum(left as usize - 1)
    }

    fn add(&mut self, i: usize, v: i32) {
        let mut j = i + 1;
        while j < self.tree.len() {
            self.tree[j] += v;
            j += Self::lsb(j);
        }
    }

    fn prefix_sum(&self, i: usize) -> i32 {
        let mut j = i + 1;
        let mut sum = 0;
        while j != 0 {
            sum += self.tree[j];
            j -= Self::lsb(j);
        }
        sum
    }

    fn lsb(x: usize) -> usize {
        let x = x as i32;
        (x & -x) as _
    }
}
