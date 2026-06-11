//! Binary Space Partitioning (BSP) tree for 2D space.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub a: [f64; 2],
    pub b: [f64; 2],
}

impl Line {
    pub fn new(a: [f64; 2], b: [f64; 2]) -> Self {
        Self { a, b }
    }

    /// Evaluate which side of this line a point is on.
    /// Returns negative, zero, or positive.
    fn side(&self, p: [f64; 2]) -> f64 {
        let dx = self.b[0] - self.a[0];
        let dy = self.b[1] - self.a[1];
        (p[0] - self.a[0]) * dy - (p[1] - self.a[1]) * dx
    }

    /// Clip this line segment against another line, returning (front, back) parts.
    fn split_by(&self, divider: &Line) -> (Option<Line>, Option<Line>) {
        let d_a = divider.side(self.a);
        let d_b = divider.side(self.b);

        let a_front = d_a >= 0.0;
        let b_front = d_b >= 0.0;

        if a_front && b_front {
            return (Some(self.clone()), None);
        }
        if !a_front && !b_front {
            return (None, Some(self.clone()));
        }

        // Compute intersection
        let t = d_a / (d_a - d_b);
        let ix = self.a[0] + t * (self.b[0] - self.a[0]);
        let iy = self.a[1] + t * (self.b[1] - self.a[1]);
        let mid = [ix, iy];

        if a_front {
            (Some(Line::new(self.a, mid)), Some(Line::new(mid, self.b)))
        } else {
            (Some(Line::new(mid, self.b)), Some(Line::new(self.a, mid)))
        }
    }
}

#[derive(Clone)]
pub enum BspNode {
    Leaf {
        lines: Vec<Line>,
    },
    Branch {
        splitter: Line,
        front: Box<BspNode>,
        back: Box<BspNode>,
    },
}

impl BspNode {
    pub fn new(lines: Vec<Line>) -> Self {
        Self::build(lines)
    }

    fn build(mut lines: Vec<Line>) -> Self {
        if lines.is_empty() {
            return BspNode::Leaf { lines: Vec::new() };
        }
        if lines.len() == 1 {
            return BspNode::Leaf { lines };
        }

        // Pick the first line as splitter
        let splitter = lines.remove(0);
        let mut front = Vec::new();
        let mut back = Vec::new();

        for line in lines {
            let (f, b) = line.split_by(&splitter);
            if let Some(fl) = f {
                front.push(fl);
            }
            if let Some(bl) = b {
                back.push(bl);
            }
        }

        // Also put the splitter in front
        front.push(splitter.clone());

        BspNode::Branch {
            splitter,
            front: Box::new(Self::build(front)),
            back: Box::new(Self::build(back)),
        }
    }

    /// Collect all lines in front-to-back order from a viewer position.
    pub fn collect_front_to_back(&self, viewer: [f64; 2], out: &mut Vec<Line>) {
        match self {
            BspNode::Leaf { lines } => out.extend(lines.iter().cloned()),
            BspNode::Branch { splitter, front, back } => {
                if splitter.side(viewer) >= 0.0 {
                    front.collect_front_to_back(viewer, out);
                    back.collect_front_to_back(viewer, out);
                } else {
                    back.collect_front_to_back(viewer, out);
                    front.collect_front_to_back(viewer, out);
                }
            }
        }
    }

    /// Count total nodes in the tree.
    pub fn node_count(&self) -> usize {
        match self {
            BspNode::Leaf { .. } => 1,
            BspNode::Branch { front, back, .. } => 1 + front.node_count() + back.node_count(),
        }
    }
}

impl fmt::Debug for BspNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BspNode::Leaf { lines } => f.debug_struct("Leaf").field("lines", &lines.len()).finish(),
            BspNode::Branch { splitter, front, back } => f.debug_struct("Branch")
                .field("splitter", splitter)
                .field("front", &**front)
                .field("back", &**back)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_bsp() {
        let lines = vec![
            Line::new([0.0, 0.0], [10.0, 0.0]),
            Line::new([5.0, -5.0], [5.0, 5.0]),
            Line::new([0.0, 3.0], [10.0, 3.0]),
        ];
        let tree = BspNode::new(lines);
        assert!(tree.node_count() >= 1);

        let mut ordered = Vec::new();
        tree.collect_front_to_back([0.0, -10.0], &mut ordered);
        assert!(!ordered.is_empty());
    }
}
