//! Core BSP node data structure.

/// Classification of a polygon relative to a partition plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneClass {
    Front,
    Back,
    OnPlane,
    Spanning,
}

/// A 2D line segment used as a partition plane.
#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub a: (f64, f64),
    pub b: (f64, f64),
}

impl Line2D {
    pub fn new(a: (f64, f64), b: (f64, f64)) -> Self {
        Self { a, b }
    }

    /// Normal direction (perpendicular to line, pointing "left" of a→b).
    pub fn normal(&self) -> (f64, f64) {
        let dx = self.b.0 - self.a.0;
        let dy = self.b.1 - self.a.1;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-10 {
            (0.0, 1.0)
        } else {
            (-dy / len, dx / len)
        }
    }

    /// Signed distance from a point to the line. Positive = front.
    pub fn signed_distance(&self, p: (f64, f64)) -> f64 {
        let (nx, ny) = self.normal();
        (p.0 - self.a.0) * nx + (p.1 - self.a.1) * ny
    }
}

/// A convex polygon in 2D space.
#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<(f64, f64)>,
}

impl Polygon {
    pub fn new(vertices: Vec<(f64, f64)>) -> Self {
        Self { vertices }
    }

    /// Classify this polygon relative to a partition line.
    pub fn classify(&self, line: &Line2D) -> PlaneClass {
        let mut front = 0;
        let mut back = 0;
        let mut _on = 0;
        let eps = 1e-8;

        for &v in &self.vertices {
            let d = line.signed_distance(v);
            if d > eps {
                front += 1;
            } else if d < -eps {
                back += 1;
            } else {
                _on += 1;
            }
        }

        if front > 0 && back > 0 {
            PlaneClass::Spanning
        } else if front > 0 {
            PlaneClass::Front
        } else if back > 0 {
            PlaneClass::Back
        } else {
            PlaneClass::OnPlane
        }
    }
}

/// A node in the BSP tree.
#[derive(Debug)]
pub enum BspNode {
    /// Leaf node containing coplanar polygons.
    Leaf {
        polygons: Vec<Polygon>,
        id: usize,
    },
    /// Internal node with a partition plane.
    Internal {
        plane: Line2D,
        front: Box<BspNode>,
        back: Box<BspNode>,
        coplanar: Vec<Polygon>,
    },
}

impl BspNode {
    pub fn new_leaf(polygons: Vec<Polygon>, id: usize) -> Self {
        BspNode::Leaf { polygons, id }
    }

    pub fn new_internal(plane: Line2D, front: BspNode, back: BspNode, coplanar: Vec<Polygon>) -> Self {
        BspNode::Internal {
            plane,
            front: Box::new(front),
            back: Box::new(back),
            coplanar,
        }
    }

    /// Count total polygons in the tree.
    pub fn count_polygons(&self) -> usize {
        match self {
            BspNode::Leaf { polygons, .. } => polygons.len(),
            BspNode::Internal { front, back, coplanar, .. } => {
                front.count_polygons() + back.count_polygons() + coplanar.len()
            }
        }
    }

    /// Count the depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            BspNode::Leaf { .. } => 1,
            BspNode::Internal { front, back, .. } => {
                1 + front.depth().max(back.depth())
            }
        }
    }

    /// Collect all leaf IDs.
    pub fn leaf_ids(&self) -> Vec<usize> {
        match self {
            BspNode::Leaf { id, .. } => vec![*id],
            BspNode::Internal { front, back, .. } => {
                let mut ids = front.leaf_ids();
                ids.extend(back.leaf_ids());
                ids
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_normal() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let (nx, ny) = line.normal();
        assert!((nx - 0.0).abs() < 1e-10);
        assert!((ny - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_signed_distance() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        assert!(line.signed_distance((0.0, 5.0)) > 0.0);
        assert!(line.signed_distance((0.0, -5.0)) < 0.0);
        assert!(line.signed_distance((3.0, 0.0)).abs() < 1e-10);
    }

    #[test]
    fn test_polygon_classify_front() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let poly = Polygon::new(vec![(0.0, 5.0), (1.0, 5.0), (0.5, 10.0)]);
        assert_eq!(poly.classify(&line), PlaneClass::Front);
    }

    #[test]
    fn test_polygon_classify_spanning() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let poly = Polygon::new(vec![(0.0, -5.0), (1.0, 5.0), (2.0, -5.0)]);
        assert_eq!(poly.classify(&line), PlaneClass::Spanning);
    }

    #[test]
    fn test_leaf_node() {
        let leaf = BspNode::new_leaf(vec![Polygon::new(vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)])], 0);
        assert_eq!(leaf.count_polygons(), 1);
        assert_eq!(leaf.depth(), 1);
    }

    #[test]
    fn test_internal_depth() {
        let l = BspNode::new_leaf(vec![], 0);
        let r = BspNode::new_leaf(vec![], 1);
        let root = BspNode::new_internal(Line2D::new((0.0, 0.0), (1.0, 0.0)), l, r, vec![]);
        assert_eq!(root.depth(), 2);
        assert_eq!(root.leaf_ids(), vec![0, 1]);
    }
}
