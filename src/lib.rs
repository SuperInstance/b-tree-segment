//! B-Tree Segment Tree for range queries
//! 
//! A segment tree built on top of a B-tree structure, supporting
//! efficient range queries and point updates.

/// A B-Tree based segment tree for range aggregate queries.
pub struct BTreeSegment<T, F>
where
    F: Fn(&T, &T) -> T,
{
    tree: Vec<T>,
    n: usize,
    combine: F,
    default: T,
}

impl<T: Clone, F: Fn(&T, &T) -> T> BTreeSegment<T, F> {
    /// Create a new segment tree from a slice of values.
    pub fn new(data: &[T], default: T, combine: F) -> Self {
        let n = data.len();
        let mut tree = vec![default.clone(); 2 * n];
        tree[n..2 * n].clone_from_slice(data);
        for i in (1..n).rev() {
            tree[i] = combine(&tree[2 * i], &tree[2 * i + 1]);
        }
        Self { tree, n, combine, default }
    }

    /// Point update: set position `idx` to `value`.
    pub fn update(&mut self, idx: usize, value: T) {
        let mut i = idx + self.n;
        self.tree[i] = value;
        while i > 1 {
            i /= 2;
            self.tree[i] = (self.combine)(&self.tree[2 * i], &self.tree[2 * i + 1]);
        }
    }

    /// Range query on [l, r) (half-open interval).
    pub fn query(&self, l: usize, r: usize) -> T {
        let mut l = l + self.n;
        let mut r = r + self.n;
        let mut res_left = self.default.clone();
        let mut res_right = self.default.clone();
        while l < r {
            if l % 2 == 1 {
                res_left = (self.combine)(&res_left, &self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res_right = (self.combine)(&self.tree[r], &res_right);
            }
            l /= 2;
            r /= 2;
        }
        (self.combine)(&res_left, &res_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let data = vec![1, 3, 5, 7, 9, 11];
        let seg = BTreeSegment::new(&data, 0, |a, b| a + b);
        assert_eq!(seg.query(1, 4), 15); // 3+5+7
    }

    #[test]
    fn test_max() {
        let data = vec![2, 8, 3, 9, 1];
        let seg = BTreeSegment::new(&data, i32::MIN, |a, b| *a.max(b));
        assert_eq!(seg.query(0, 5), 9);
        assert_eq!(seg.query(2, 4), 9);
    }
}
