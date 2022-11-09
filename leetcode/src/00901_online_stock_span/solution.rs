struct Solution;

#[derive(Default)]
struct StockSpanner {
    st: Vec<(i32, usize)>,
}

impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while let Some(&(top, cnt)) = self.st.last() {
            if price < top {
                break;
            }
            res += cnt;
            self.st.pop();
        }
        self.st.push((price, res));
        res as _
    }
}
