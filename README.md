# b-tree-segment

**Iterative segment tree for range-aggregate queries — O(log n) point updates, O(log n) range queries, zero allocation per query.**

A segment tree is a binary tree data structure that stores aggregate information about array intervals. Each leaf holds one array element; each internal node stores the combination (sum, max, min, etc.) of its children. Unlike a recursive segment tree, this implementation uses the **iterative bottom-up layout** popularized by competitive programming: the tree is stored in a flat `Vec<T>` of size 2n, where indices [n, 2n) hold leaves and indices [1, n) hold internal nodes.

## Why It Matters

Range queries are fundamental to computational geometry, database indexing, signal processing, and bioinformatics. The segment tree is the optimal solution when you need both **point updates** and **range queries** on the same array:

- **Database range scans** — "sum of records in time window [t₁, t₂)" is a range sum query
- **Genome analysis** — "max conservation score in chromosome region" is a range max query
- **Financial time series** — rolling window statistics require repeated range aggregation
- **Image processing** — 2D prefix sums extend segment tree ideas to images

Alternatives trade off different axes:

| Structure | Build | Update | Range Query | Notes |
|-----------|-------|--------|-------------|-------|
| Segment tree (this crate) | O(n) | O(log n) | O(log n) | General, any associative op |
| Fenwick tree (BIT) | O(n) | O(log n) | O(log n) | Only invertible ops (sum) |
| Sparse table | O(n log n) | O(1)* | O(1) | *No updates — static data only |
| Sqrt decomposition | O(n) | O(1) | O(√n) | Simpler but slower queries |

The segment tree's advantage: **any associative operation** works (sum, min, max, gcd, matrix multiply, etc.) and it supports both updates and queries efficiently.

## How It Works

### Array Layout

For input array `data[0..n]`, the tree is stored as:

```
Index:  1    2    3    4    5  ...  n   n+1  ...  2n-1
       root                              data[0] data[1] ...
```

- **Leaves**: `tree[n + i] = data[i]` for i ∈ [0, n)
- **Internal nodes**: `tree[i] = combine(tree[2i], tree[2i+1])` for i ∈ [1, n)

Build is bottom-up: fill leaves, then iterate i from n-1 down to 1, combining children.

### Point Update

To set `data[idx] = value`:

```
i = idx + n
tree[i] = value
while i > 1:
    i = i / 2
    tree[i] = combine(tree[2i], tree[2i+1])
```

This walks from leaf to root, updating O(log n) nodes.

### Range Query [l, r)

The iterative query processes the range [l + n, r + n) from bottom to top:

```
l = l + n; r = r + n
result_left = default; result_right = default
while l < r:
    if l is odd:  result_left = combine(result_left, tree[l]);  l += 1
    if r is odd:  r -= 1;  result_right = combine(tree[r], result_right)
    l = l / 2;  r = r / 2
return combine(result_left, result_right)
```

The key insight: when `l` is odd, `tree[l]` is the right child of its parent, so its parent's range extends beyond our query. We must process `tree[l]` individually and advance. Similarly for `r`.

**Why two accumulators?** Left is combined left-to-right; right is combined right-to-left. This preserves order for non-commutative operations (e.g., matrix multiplication, string concatenation). The final combine merges both sides correctly.

### Correctness Invariant

The `combine` function must be **associative**: combine(a, combine(b, c)) = combine(combine(a, b), c). It need not be commutative.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `new(data, default, combine)` | O(n) | O(2n) |
| `update(idx, value)` | O(log n) | O(1) |
| `query(l, r)` | O(log n) | O(1) |

The iterative query visits at most 4⌈log₂ n⌉ nodes per query.

## Quick Start

```rust
use b_tree_segment::BTreeSegment;

// Range sum queries
let data = vec![1, 3, 5, 7, 9, 11];
let mut seg = BTreeSegment::new(&data, 0, |a, b| a + b);

assert_eq!(seg.query(1, 4), 15);  // 3 + 5 + 7
assert_eq!(seg.query(0, 6), 36);  // sum of all

// Point update
seg.update(2, 10);  // data[2] = 10 (was 5)
assert_eq!(seg.query(1, 4), 20);  // 3 + 10 + 7

// Range max
let data2 = vec![2, 8, 3, 9, 1];
let seg_max = BTreeSegment::new(&data2, i32::MIN, |a, b| *a.max(b));
assert_eq!(seg_max.query(0, 5), 9);
assert_eq!(seg_max.query(2, 4), 9);
```

## API

- **`BTreeSegment<T, F>`** — Generic segment tree parameterized by:
  - `T: Clone` — Element type
  - `F: Fn(&T, &T) -> T` — Associative combine function
- **`new(data: &[T], default: T, combine: F)`** — Build tree from slice, O(n)
- **`update(idx: usize, value: T)`** — Point update, O(log n)
- **`query(l: usize, r: usize) -> T`** — Aggregate over [l, r), O(log n)

## Architecture Notes

The segment tree embodies the γ+η=C identity at the data structure level: γ (generative capacity) is the variety of combine functions the tree supports (any associative op), while η (evaluative depth) is the query/update efficiency (O(log n) both). C = the set of solvable problems. A Fenwick tree has higher η for invertible ops (simpler code) but lower γ (can't do max/gcd). A sparse table has maximum η for static queries but zero γ (no updates).

## References

1. de Berg, M. et al. (2008). *Computational Geometry* (3rd ed.), §10.3. — Segment trees in computational geometry.
2. Knuth, D. (1998). *The Art of Computer Programming, Vol. 3: Sorting and Searching* (2nd ed.). — Binary tree structures.
3. Al.Cash's competitive programming blog (2014). "Efficient and easy segment trees." — The iterative layout used here.
4. Bentley, J. (1977). "Solutions to Klee's Rectangle Problems." Technical Report, CMU. — Origins of segment trees for orthogonal range queries.

## License

MIT
