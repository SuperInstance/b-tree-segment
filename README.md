# B-Tree Segment Tree

**A Rust library for range-query segment trees** — a data structure that supports O(log n) point updates and O(log n) range aggregate queries on an array, using a flat 2n-size binary tree representation.

## Why It Matters

The segment tree is one of the most versatile data structures in competitive programming and systems engineering. It answers questions like "what's the sum of elements in positions 3–7?" or "what's the minimum value in the range [0, 1000)?" in O(log n) time, with O(log n) updates when a single element changes.

Unlike a prefix-sum array (which supports O(1) range queries but O(n) updates), a segment tree handles both in logarithmic time. This makes it ideal for:

- **Real-time analytics** — running sums/averages over sliding windows
- **Financial systems** — portfolio value over configurable ranges
- **Game engines** — spatial queries over entity attributes
- **Database internals** — range aggregates for query planning

The "B-tree" in the name refers to the B+tree-style bottom-up storage: data lives in the leaves (indices n..2n), and internal nodes (1..n) store aggregated results of their children. This is the same layout used by Fenwick trees (BIT) but more general — any associative combine function works.

## How It Works

**Construction** O(n): Data is placed at positions `n..2n` in a flat array. Internal nodes at `i` (for `i` in `1..n`, computed bottom-up) store `combine(tree[2i], tree[2i+1])`. The combine function is generic — pass `|a, b| a + b` for sum, `|a, b| *a.max(b)` for max, etc.

**Point Update** O(log n): Set `tree[idx + n] = value`, then walk up from `idx + n` to the root, recomputing each parent as `combine(tree[2i], tree[2i+1])`.

**Range Query** O(log n) on `[l, r)` (half-open): Start with two pointers at `l + n` and `r + n`. At each level, if the left pointer is a right child, include it and move right; if the right pointer is a right child, move left and include the new right. Combine accumulated left/right results. This traverses at most 2·log(n) nodes.

## Quick Start

```rust
use b_tree_segment::BTreeSegment;

let data = vec![1, 3, 5, 7, 9, 11];

// Sum segment tree
let mut seg = BTreeSegment::new(&data, 0, |a, b| a + b);
assert_eq!(seg.query(1, 4), 15); // 3 + 5 + 7

// Update position 2 to 10
seg.update(2, 10);
assert_eq!(seg.query(1, 4), 20); // 3 + 10 + 7

// Max segment tree
let max_seg = BTreeSegment::new(&data, i32::MIN, |a, b| *a.max(b));
assert_eq!(max_seg.query(0, 6), 11);
```

## API

- **`BTreeSegment<T, F>`** — Generic segment tree parameterized by value type and combine function
  - `new(data, default, combine)` — Build from a slice, O(n)
  - `update(idx, value)` — Point update, O(log n)
  - `query(l, r)` — Range aggregate on `[l, r)`, O(log n)

## Architecture Notes

Provides the range-query primitive for SuperInstance analytics services. The generic combine function allows reuse across sum, min, max, GCD, and custom aggregation pipelines. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
