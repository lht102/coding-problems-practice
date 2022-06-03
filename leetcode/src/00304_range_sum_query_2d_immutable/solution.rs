struct NumMatrix {
    psum: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut psum = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                psum[i][j] =
                    psum[i - 1][j] + psum[i][j - 1] + matrix[i - 1][j - 1] - 2 * psum[i - 1][j - 1];
            }
        }
        Self { psum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        self.psum[row2 + 1][col2 + 1] - self.psum[row2 + 1][col1] - self.psum[row1][col2 + 1]
            + self.psum[row1][col1]
    }
}
