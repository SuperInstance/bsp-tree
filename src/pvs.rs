//! Potentially Visible Set (PVS) computation.

use crate::node::BspNode;
use crate::portal::generate_portals;
use std::collections::{HashMap, HashSet};

/// Compute the potentially visible set for each leaf.
/// Returns a map from leaf ID to the set of potentially visible leaf IDs.
pub fn compute_pvs(root: &BspNode) -> HashMap<usize, HashSet<usize>> {
    let portals = generate_portals(root);
    let leaf_ids = root.leaf_ids();
    let mut pvs: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Initialize PVS for each leaf
    for &id in &leaf_ids {
        pvs.insert(id, HashSet::new());
    }

    // Two leaves are potentially visible of each other if connected by a portal
    for portal in &portals {
        pvs.get_mut(&portal.front_leaf)
            .map(|s| s.insert(portal.back_leaf));
        pvs.get_mut(&portal.back_leaf)
            .map(|s| s.insert(portal.front_leaf));
    }

    // Transitive closure: if A sees B and B sees C, A might see C
    let _changed = true;
    let max_iterations = leaf_ids.len() * leaf_ids.len();
    for _ in 0..max_iterations {
        let mut did_change = false;
        let mut additions: Vec<(usize, usize)> = Vec::new();
            for &leaf in &leaf_ids {
                let visible = pvs.get(&leaf).cloned().unwrap_or_default();
                for &v in &visible {
                    if let Some(v_vis) = pvs.get(&v) {
                        for &vv in v_vis.iter() {
                            if vv != leaf && !visible.contains(&vv) {
                                additions.push((leaf, vv));
                        }
                    }
                }
            }
        }
        for (leaf, vis) in &additions {
            if let Some(s) = pvs.get_mut(leaf) {
                if s.insert(*vis) {
                    did_change = true;
                }
            }
        }
        if !did_change {
            break;
        }
    }

    pvs
}

/// Check if leaf `from` can potentially see leaf `to`.
pub fn is_potentially_visible(pvs: &HashMap<usize, HashSet<usize>>, from: usize, to: usize) -> bool {
    pvs.get(&from).is_some_and(|s| s.contains(&to))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{BspNode, Line2D, Polygon};

    fn make_square(x: f64, y: f64, s: f64) -> Polygon {
        Polygon::new(vec![(x, y), (x + s, y), (x + s, y + s), (x, y + s)])
    }

    #[test]
    fn test_pvs_single_leaf() {
        let leaf = BspNode::new_leaf(vec![], 0);
        let pvs = compute_pvs(&leaf);
        assert_eq!(pvs[&0].len(), 0);
    }

    #[test]
    fn test_pvs_two_leaves() {
        let leaf0 = BspNode::new_leaf(vec![], 0);
        let leaf1 = BspNode::new_leaf(vec![], 1);
        let root = BspNode::new_internal(
            Line2D::new((0.0, 0.0), (0.0, 1.0)),
            leaf0,
            leaf1,
            vec![],
        );
        let pvs = compute_pvs(&root);
        assert!(is_potentially_visible(&pvs, 0, 1));
        assert!(is_potentially_visible(&pvs, 1, 0));
    }

    #[test]
    fn test_pvs_symmetry() {
        let leaf0 = BspNode::new_leaf(vec![], 0);
        let leaf1 = BspNode::new_leaf(vec![], 1);
        let leaf2 = BspNode::new_leaf(vec![], 2);
        let right = BspNode::new_internal(
            Line2D::new((5.0, 0.0), (5.0, 1.0)),
            leaf1,
            leaf2,
            vec![],
        );
        let root = BspNode::new_internal(
            Line2D::new((0.0, 0.0), (0.0, 1.0)),
            leaf0,
            right,
            vec![],
        );
        let pvs = compute_pvs(&root);
        // Check symmetry
        for (&from, visible) in &pvs {
            for &to in visible {
                assert!(is_potentially_visible(&pvs, to, from),
                    "PVS not symmetric: {} sees {} but not vice versa", from, to);
            }
        }
    }

    #[test]
    fn test_pvs_not_self_visible() {
        let leaf0 = BspNode::new_leaf(vec![], 0);
        let leaf1 = BspNode::new_leaf(vec![], 1);
        let root = BspNode::new_internal(
            Line2D::new((0.0, 0.0), (0.0, 1.0)),
            leaf0,
            leaf1,
            vec![],
        );
        let pvs = compute_pvs(&root);
        assert!(!is_potentially_visible(&pvs, 0, 0));
        assert!(!is_potentially_visible(&pvs, 1, 1));
    }

    #[test]
    fn test_pvs_with_polygons() {
        let tree = crate::bsp::BspTree::build(vec![
            make_square(0.0, 0.0, 1.0),
            make_square(5.0, 0.0, 1.0),
        ]);
        let pvs = compute_pvs(tree.root());
        // Should have entries for all leaves
        assert!(!pvs.is_empty());
    }
}
