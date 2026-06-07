//! Polygon splitting along a partition line.

use crate::node::{Line2D, Polygon};

/// Split a polygon along a line, returning (front, back) parts.
/// If the polygon is entirely on one side, returns it in that slot with None for the other.
pub fn split_polygon(poly: &Polygon, line: &Line2D) -> (Option<Polygon>, Option<Polygon>) {
    let eps = 1e-8;
    let n = poly.vertices.len();
    if n < 3 {
        return (None, None);
    }

    let mut front_verts = Vec::new();
    let mut back_verts = Vec::new();

    for i in 0..n {
        let j = (i + 1) % n;
        let vi = poly.vertices[i];
        let vj = poly.vertices[j];
        let di = line.signed_distance(vi);
        let dj = line.signed_distance(vj);

        if di > eps {
            front_verts.push(vi);
        } else if di < -eps {
            back_verts.push(vi);
        } else {
            front_verts.push(vi);
            back_verts.push(vi);
        }

        // Check if edge crosses the line
        if (di > eps && dj < -eps) || (di < -eps && dj > eps) {
            let t = di / (di - dj);
            let ix = vi.0 + t * (vj.0 - vi.0);
            let iy = vi.1 + t * (vj.1 - vi.1);
            front_verts.push((ix, iy));
            back_verts.push((ix, iy));
        }
    }

    let make_poly = |v: Vec<(f64, f64)>| {
        if v.len() >= 3 {
            Some(Polygon::new(v))
        } else {
            None
        }
    };

    (make_poly(front_verts), make_poly(back_verts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_spanning_polygon() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let poly = Polygon::new(vec![(-2.0, -2.0), (2.0, -2.0), (0.0, 2.0)]);
        let (front, back) = split_polygon(&poly, &line);
        assert!(front.is_some());
        assert!(back.is_some());
    }

    #[test]
    fn test_split_front_only() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let poly = Polygon::new(vec![(0.0, 1.0), (1.0, 1.0), (0.5, 3.0)]);
        let (front, back) = split_polygon(&poly, &line);
        assert!(front.is_some());
        assert!(back.is_none());
    }

    #[test]
    fn test_split_back_only() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let poly = Polygon::new(vec![(0.0, -3.0), (1.0, -3.0), (0.5, -1.0)]);
        let (front, back) = split_polygon(&poly, &line);
        assert!(front.is_none());
        assert!(back.is_some());
    }

    #[test]
    fn test_split_preserves_total_area() {
        let line = Line2D::new((0.0, 0.0), (1.0, 0.0));
        let poly = Polygon::new(vec![(-4.0, -4.0), (4.0, -4.0), (0.0, 4.0)]);
        let orig_area = poly_area(&poly);
        let (front, back) = split_polygon(&poly, &line);
        let sum_area = front.as_ref().map_or(0.0, |p| poly_area(p))
            + back.as_ref().map_or(0.0, |p| poly_area(p));
        assert!((orig_area - sum_area).abs() < 0.1, "orig={} sum={}", orig_area, sum_area);
    }

    fn poly_area(p: &Polygon) -> f64 {
        let n = p.vertices.len();
        let mut a = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            a += p.vertices[i].0 * p.vertices[j].1 - p.vertices[j].0 * p.vertices[i].1;
        }
        a.abs() / 2.0
    }
}
