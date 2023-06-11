struct Solution;

struct SnapshotArray {
    snap_id: i32,
    records: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            snap_id: 0,
            records: vec![vec![(0, 0)]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.records[index as usize].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        let snap_id = self.snap_id;
        self.snap_id += 1;
        snap_id
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let i = index as usize;
        let idx = self.records[i].partition_point(|x| x.0 <= snap_id);
        self.records[i][idx - 1].1
    }
}
