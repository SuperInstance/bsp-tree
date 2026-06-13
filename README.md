# BSP Tree

**BSP Tree** (Binary Space Partitioning) is a Rust library implementing a 2D BSP tree for spatial partitioning of line segments — providing front-to-back ordering from any viewpoint, used in rendering, collision detection, and constructive solid geometry.

## Why It Matters

BSP trees were the rendering backbone of every major 3D game engine from Doom (1993) through Quake III Arena (1999). The core idea — recursively splitting space by hyperplanes to create an ordered traversal from any viewpoint — eliminates the need for a z-buffer and enables perfect visibility sorting. Beyond rendering, BSP trees are used in CAD systems for boolean operations on polygons, in robotics for motion planning (partitioning free space), and in collision detection for broad-phase culling. The structure's ability to produce a viewpoint-dependent ordering in O(n) traversal time (after O(n² log n) preprocessing) makes it uniquely valuable when the same geometry is viewed from many different angles.

## How It Works

**Construction:** The tree is built recursively from a set of line segments:

```
build(lines):
  if len(lines) ≤ 1: return Leaf(lines)
  splitter = lines[0]        // pick a splitting line
  front = []; back = []
  for each line in lines[1:]:
    (f, b) = line.split_by(splitter)
    if f: front.append(f)
    if b: back.append(f)
  front.append(splitter)     // splitter goes in front
  return Branch(splitter, build(front), build(back))
```

**Line splitting:** A line segment AB is classified against a divider by computing the cross product:

```
side(P) = (P.x − A.x)(B.y − A.y) − (P.y − A.y)(B.x − A.x)
```

- side > 0: P is in front (left) of the line
- side < 0: P is behind (right)
- side = 0: P is on the line

If the two endpoints of a segment are on opposite sides, the segment is split at the intersection point using parametric interpolation:

```
t = side(A) / (side(A) − side(B))
intersection = A + t × (B − A)
```

**Front-to-back traversal:** Given a viewpoint V:

```
traverse(node, V):
  if Leaf: output all lines
  if Branch:
    if splitter.side(V) ≥ 0:  // viewer in front
      traverse(front, V)
      traverse(back, V)
    else:                      // viewer behind
      traverse(back, V)
      traverse(front, V)
```

This produces an O(n) ordered output — no sorting needed.

**Complexity:**

| Operation | Time | Notes |
|-----------|------|-------|
| Construction | O(n² log n) worst | Depends on splitter choice |
| Front-to-back traversal | O(n) | For n lines |
| Point classification | O(log n) | One comparison per level |
| Node count | O(n) | Each split adds ≤ 1 node |

## Quick Start

```rust
use bsp_tree::{Line, BspNode};

fn main() {
    let lines = vec![
        Line::new([0.0, 0.0], [10.0, 0.0]),
        Line::new([5.0, -5.0], [5.0, 5.0]),
        Line::new([0.0, 3.0], [10.0, 3.0]),
    ];
    let tree = BspNode::new(lines);
    println!("Tree nodes: {}", tree.node_count());

    let mut ordered = Vec::new();
    tree.collect_front_to_back([0.0, -10.0], &mut ordered);
    println!("Front-to-back from (0,-10): {} lines", ordered.len());
}
```

## API

| Type/Method | Description |
|-------------|-------------|
| `Line` | 2D line segment with split/classify operations |
| `BspNode` | Recursive tree: Branch or Leaf |
| `BspNode::new` | Build tree from list of lines |
| `collect_front_to_back` | Viewpoint-ordered traversal |
| `node_count` | Total nodes in tree |

## Architecture Notes

The BSP Tree provides the **spatial partitioning layer** for the SuperInstance fleet's simulation environment. Within γ + η = C, the BSP tree partitions physical space (γ-layer) so that conservation-law observers (η-layer) can efficiently query spatial regions — for example, counting agents in a sub-region to verify local avoidance-ratio conservation.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

**Splitter selection heuristic:** This implementation picks the first line as the splitter — simple but produces unbalanced trees for sorted input. Better heuristics include: (1) pick the line that minimizes split count (minimize O(n²) splits), (2) pick the median-angle line (balance the tree), or (3) random selection (probabilistic balance). The optimal splitter choice is NP-hard in general.

**3D extension:** Extending to 3D requires splitting polygons (triangles) by planes rather than lines by lines. The cross product generalizes to the plane normal, and the parametric intersection computation extends naturally. The front-to-back traversal remains O(n) for a balanced tree.

## References

1. Fuchs, H., Kedem, Z.M., & Naylor, B.F. (1980). "On Visible Surface Generation by A Priori Tree Structures." *SIGGRAPH*.
2. Gordon, D. & Chen, S. (1991). "Front-to-Back Display of BSP Trees." *IEEE Computer Graphics and Applications*.

## License

MIT
