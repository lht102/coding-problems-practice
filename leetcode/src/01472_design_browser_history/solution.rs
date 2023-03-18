struct BrowserHistory {
    urls: Vec<String>,
    cur_url_idx: usize,
    last_url_idx: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            urls: Vec::from([homepage]),
            cur_url_idx: 0,
            last_url_idx: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.cur_url_idx += 1;
        if self.urls.len() > self.cur_url_idx {
            self.urls[self.cur_url_idx] = url;
        } else {
            self.urls.push(url);
        }
        self.last_url_idx = self.cur_url_idx;
    }

    fn back(&mut self, steps: i32) -> String {
        self.cur_url_idx = self.cur_url_idx.saturating_sub(steps as _);
        self.urls[self.cur_url_idx].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.cur_url_idx = self.last_url_idx.min(self.cur_url_idx + steps as usize);
        self.urls[self.cur_url_idx].clone()
    }
}
