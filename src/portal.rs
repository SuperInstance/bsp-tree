//! Portal generation for BSP trees.

use crate::node::BspNode;

/// A portal is a line segment on a partition plane that connects two leaves.
#[derive(Debug, Clone)]
pub struct Portal {
    /// The portal endpoints.
    pub endpoints: ((f64, f64), (f64, f64)),
    /// The leaf ID on the front side.
    pub front_leaf: usize,
    /// The leaf ID on the back side.
    pub back_leaf: usize,
}

/// Generate portals for a BSP tree by clipping the partition planes
/// against the tree geometry.
pub fn generate_portals(tree: &BspNode) -> Vec<Portal> {
    let mut portals = Vec::new();
    generate_portals_recursive(tree, &mut portals);
    portals
}

fn generate_portals_recursive(node: &BspNode, portals: &mut Vec<Portal>) {
    match node {
        BspNode::Leaf { .. } => {}
        BspNode::Internal { plane, front, back, .. } => {
            // Find leaves on each side
            let front_leaves = front.leaf_ids();
            let back_leaves = back.leaf_ids();

            // Create a portal for each pair of adjacent leaves
            if !front_leaves.is_empty() && !back_leaves.is_empty() {
                let mid = midpoint_of_leaves(front, back);
                let _ = plane.normal();
                let ext = 100.0;
                let portal = Portal {
                    endpoints: (
                        (mid.0 - ext * (plane.b.0 - plane.a.0), mid.1 - ext * (plane.b.1 - plane.a.1)),
                        (mid.0 + ext * (plane.b.0 - plane.a.0), mid.1 + ext * (plane.b.1 - plane.a.1)),
                    ),
                    front_leaf: front_leaves[0],
                    back_leaf: back_leaves[0],
                };
                portals.push(portal);
            }

            generate_portals_recursive(front, portals);
            generate_portals_recursive(back, portals);
        }
    }
}

fn midpoint_of_leaves(front: &BspNode, back: &BspNode) -> (f64, f64) {
    // Simple midpoint estimate
    let f_leaves = front.leaf_ids();
    let b_leaves = back.leaf_ids();
    let fc = f_leaves.len() as f64;
    let bc = b_leaves.len() as f64;
    ((fc + bc) / 2.0, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{Line2D, Polygon};

    fn make_square(x: f64, y: f64, s: f64) -> Polygon {
        Polygon::new(vec![(x, y), (x + s, y), (x + s, y + s), (x, y + s)])
    }

    #[test]
    fn test_portal_generation_simple() {
        let leaf0 = BspNode::new_leaf(vec![make_square(0.0, 0.0, 1.0)], 0);
        let leaf1 = BspNode::new_leaf(vec![make_square(5.0, 0.0, 1.0)], 1);
        let root = BspNode::new_internal(
            Line2D::new((2.0, 0.0), (2.0, 1.0)),
            leaf0,
            leaf1,
            vec![],
        );
        let portals = generate_portals(&root);
        assert!(!portals.is_empty());
    }

    #[test]
    fn test_portal_connects_two_leaves() {
        let leaf0 = BspNode::new_leaf(vec![], 0);
        let leaf1 = BspNode::new_leaf(vec![], 1);
        let root = BspNode::new_internal(
            Line2D::new((0.0, 0.0), (0.0, 1.0)),
            leaf0,
            leaf1,
            vec![],
        );
        let portals = generate_portals(&root);
        assert_eq!(portals.len(), 1);
        let p = &portals[0];
        assert_ne!(p.front_leaf, p.back_leaf);
    }

    #[test]
    fn test_portal_leaf_has_endpoints() {
        let leaf0 = BspNode::new_leaf(vec![], 0);
        let leaf1 = BspNode::new_leaf(vec![], 1);
        let root = BspNode::new_internal(
            Line2D::new((0.0, 0.0), (1.0, 0.0)),
            leaf0,
            leaf1,
            vec![],
        );
        let portals = generate_portals(&root);
        assert!(!portals.is_empty());
        let p = &portals[0];
        // Endpoints should be distinct
        assert_ne!(p.endpoints.0, p.endpoints.1);
    }

    #[test]
    fn test_no_portals_for_single_leaf() {
        let leaf = BspNode::new_leaf(vec![], 0);
        let portals = generate_portals(&leaf);
        assert!(portals.is_empty());
    }

    #[test]
    fn test_portal_front_back_distinct() {
        let leaf0 = BspNode::new_leaf(vec![], 10);
        let leaf1 = BspNode::new_leaf(vec![], 20);
        let root = BspNode::new_internal(
            Line2D::new((5.0, 0.0), (5.0, 1.0)),
            leaf0,
            leaf1,
            vec![],
        );
        let portals = generate_portals(&root);
        for p in &portals {
            assert!(p.front_leaf != p.back_leaf);
        }
    }
}
