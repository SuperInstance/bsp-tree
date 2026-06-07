//! BSP tree construction and traversal.

use crate::node::{BspNode, Line2D, PlaneClass, Polygon};
use crate::splitter::split_polygon;

/// A BSP tree for 2D polygons.
pub struct BspTree {
    root: BspNode,
}

impl BspTree {
    /// Build a BSP tree from a list of polygons.
    pub fn build(polygons: Vec<Polygon>) -> Self {
        let mut next_id = 0;
        let root = Self::build_recursive(polygons, &mut next_id);
        BspTree { root }
    }

    fn build_recursive(mut polygons: Vec<Polygon>, next_id: &mut usize) -> BspNode {
        if polygons.is_empty() {
            return BspNode::new_leaf(vec![], *next_id);
        }

        if polygons.len() == 1 {
            let id = *next_id;
            *next_id += 1;
            return BspNode::new_leaf(polygons, id);
        }

        // Use the first polygon's first edge as partition
        let first = &polygons[0];
        let v = &first.vertices;
        let (a, b) = if v.len() >= 2 {
            (v[0], v[1])
        } else {
            ((0.0, 0.0), (1.0, 0.0))
        };
        let plane = Line2D::new(a, b);

        let partition = polygons.remove(0);
        let mut front_polys = Vec::new();
        let mut back_polys = Vec::new();
        let mut coplanar = vec![partition];

        for poly in polygons {
            match poly.classify(&plane) {
                PlaneClass::Front => front_polys.push(poly),
                PlaneClass::Back => back_polys.push(poly),
                PlaneClass::OnPlane => coplanar.push(poly),
                PlaneClass::Spanning => {
                    let (f, b) = split_polygon(&poly, &plane);
                    if let Some(fp) = f {
                        front_polys.push(fp);
                    }
                    if let Some(bp) = b {
                        back_polys.push(bp);
                    }
                }
            }
        }

        let front = Self::build_recursive(front_polys, next_id);
        let back = Self::build_recursive(back_polys, next_id);

        BspNode::new_internal(plane, front, back, coplanar)
    }

    /// Get a reference to the root node.
    pub fn root(&self) -> &BspNode {
        &self.root
    }

    /// Traverse in front-to-back order from a viewer position.
    /// Returns leaf IDs in visibility priority order.
    pub fn traverse_front_to_back(&self, viewer: (f64, f64)) -> Vec<usize> {
        let mut result = Vec::new();
        Self::ftb_traverse(&self.root, viewer, &mut result);
        result
    }

    fn ftb_traverse(node: &BspNode, viewer: (f64, f64), result: &mut Vec<usize>) {
        match node {
            BspNode::Leaf { id, .. } => {
                result.push(*id);
            }
            BspNode::Internal { plane, front, back, .. } => {
                let d = plane.signed_distance(viewer);
                if d >= 0.0 {
                    Self::ftb_traverse(front, viewer, result);
                    Self::ftb_traverse(back, viewer, result);
                } else {
                    Self::ftb_traverse(back, viewer, result);
                    Self::ftb_traverse(front, viewer, result);
                }
            }
        }
    }

    /// Traverse back-to-front (painter's algorithm order).
    pub fn traverse_back_to_front(&self, viewer: (f64, f64)) -> Vec<usize> {
        let mut ftb = self.traverse_front_to_back(viewer);
        ftb.reverse();
        ftb
    }

    /// Count total leaves.
    pub fn leaf_count(&self) -> usize {
        self.root.leaf_ids().len()
    }

    /// Get tree depth.
    pub fn depth(&self) -> usize {
        self.root.depth()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_square(x: f64, y: f64, s: f64) -> Polygon {
        Polygon::new(vec![
            (x, y),
            (x + s, y),
            (x + s, y + s),
            (x, y + s),
        ])
    }

    #[test]
    fn test_build_single_polygon() {
        let tree = BspTree::build(vec![make_square(0.0, 0.0, 1.0)]);
        assert_eq!(tree.leaf_count(), 1);
    }

    #[test]
    fn test_build_multiple_polygons() {
        let polys = vec![
            make_square(0.0, 0.0, 1.0),
            make_square(5.0, 0.0, 1.0),
            make_square(0.0, 5.0, 1.0),
        ];
        let tree = BspTree::build(polys);
        assert!(tree.leaf_count() >= 2);
    }

    #[test]
    fn test_front_to_back_ordering() {
        let polys = vec![
            make_square(0.0, 0.0, 1.0),
            make_square(10.0, 0.0, 1.0),
        ];
        let tree = BspTree::build(polys);
        let order = tree.traverse_front_to_back((0.0, 0.0));
        assert!(!order.is_empty());
    }

    #[test]
    fn test_back_to_front_reverses() {
        let polys = vec![
            make_square(0.0, 0.0, 1.0),
            make_square(5.0, 0.0, 1.0),
        ];
        let tree = BspTree::build(polys);
        let ftb = tree.traverse_front_to_back((0.0, 0.0));
        let btf = tree.traverse_back_to_front((0.0, 0.0));
        assert_eq!(ftb, btf.into_iter().rev().collect::<Vec<_>>());
    }

    #[test]
    fn test_empty_tree() {
        let tree = BspTree::build(vec![]);
        assert_eq!(tree.leaf_count(), 1); // empty leaf
    }

    #[test]
    fn test_tree_depth_grows() {
        let tree1 = BspTree::build(vec![make_square(0.0, 0.0, 1.0)]);
        let tree3 = BspTree::build(vec![
            make_square(0.0, 0.0, 1.0),
            make_square(5.0, 5.0, 1.0),
            make_square(10.0, 10.0, 1.0),
        ]);
        assert!(tree3.depth() >= tree1.depth());
    }
}
