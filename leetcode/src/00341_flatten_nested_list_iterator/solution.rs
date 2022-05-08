use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    st: Vec<IntoIter<NestedInteger>>,
    next: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            st: vec![nested_list.into_iter()],
            next: 0,
        }
    }

    fn next(&mut self) -> i32 {
        self.next
    }

    fn has_next(&mut self) -> bool {
        while !self.st.is_empty() {
            while let Some(item) = self.st.last_mut().unwrap().next() {
                match item {
                    NestedInteger::Int(num) => {
                        self.next = num;
                        return true;
                    }
                    NestedInteger::List(inner) => self.st.push(inner.into_iter()),
                }
            }
            self.st.pop();
        }
        false
    }
}
