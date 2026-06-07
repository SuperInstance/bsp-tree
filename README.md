# bsp-tree

Binary Space Partitioning tree for 2D space in pure Rust.

## Features

- BSP tree construction from polygons
- Front-to-back and back-to-front traversal
- Polygon splitting along partition lines
- Portal generation between BSP leaves
- Potentially Visible Set (PVS) computation

## Usage

```rust
use bsp_tree::{BspTree, Polygon, Line2D, compute_pvs};

let polygons = vec![
    Polygon::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
    Polygon::new(vec![(5.0, 0.0), (6.0, 0.0), (6.0, 1.0), (5.0, 1.0)]),
];
let tree = BspTree::build(polygons);
let order = tree.traverse_front_to_back((0.0, 0.0));
let pvs = compute_pvs(tree.root());
```

Zero external dependencies. Pure `std` Rust.

## License

MIT
