# BSP Tree (Binary Space Partitioning)

A **BSP tree** recursively subdivides space using hyperplanes, creating a tree where each node represents a splitting plane and leaves represent convex regions.

## Why It Matters

BSP trees powered the original Doom renderer and remain important for collision detection, hidden surface removal, and spatial ordering. Unlike BVH trees, BSP trees produce strict front-to-back ordering which enables early z-termination.

## How It Works

At each node, a hyperplane splits space into 'front' and 'back' half-spaces. Polygons on the plane stay at the node. Recursion continues until each leaf contains one polygon. Traversal order (front-to-back or back-to-front) gives painter's algorithm for free.

## Usage

```toml
[dependencies]
bsp-tree = "0.1.0"
```

```rust
use bsp_tree;

// See examples/ directory for detailed usage
```

## API

- `Line` (lib.rs)
- `BspNode` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
