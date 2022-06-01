use std::{ops::Range, rc::Rc, sync::Arc};

use crate::geometry::Point;

use super::BoundingBox;

/*

Pseudocode:

given a list of primitive references

compute their scene bbox

starting at root, do a dfs type traversal.
we want to find splits of the prim vec


best_split = None
for each axis:
    initialize B buckets
    for each primitive p:
        find b, bucket of p
        b.count++
        b.bbox.union(p.bbox)

    for each of B-1 splits, compute its cost, then compare that with best_split and assign if better

using best_split:

partition primitives by the split
assign this.left to build_bvh of left
assign this.right to build_bvh of right

*/

pub trait Bounded {
    fn bbox(&self) -> BoundingBox;
    fn surface_area(&self) -> f64;
    fn centroid(&self) -> Point<f64>;
}

impl Bounded for BoundingBox {
    fn bbox(&self) -> BoundingBox {
        return self.clone();
    }

    fn surface_area(&self) -> f64 {
        let x = self.x();
        let y = self.y();
        let z = self.z();

        (x.end - x.start) * (y.end - y.start) * (z.end - z.start)
    }

    fn centroid(&self) -> Point<f64> {
        let x = self.x();
        let y = self.y();
        let z = self.z();

        Point::new(
            (x.start + x.end) / 2.0,
            (y.start + y.end) / 2.0,
            (z.start + z.end) / 2.0,
        )
    }
}

pub struct BvhTree {
    root: BvhNode,
}

impl BvhTree {
    pub fn build(primitives: Vec<Rc<dyn Bounded>>) -> Self {
        let root = build_bvh_node(primitives);

        Self { root }
    }
}

fn full_bbox(items: &[Rc<dyn Bounded>]) -> Option<BoundingBox> {
    items.iter().map(|p| p.bbox()).reduce(|mut acc, item| {
        acc += item;
        acc
    })
}

struct Bucket {
    count: u32,
    bbox: Option<BoundingBox>,
}

const NUM_BUCKETS: usize = 8;
const COST_TRAVERSAL: f64 = 1.0 / 8.0;
const MAX_PRIMS_PER_NODE: usize = 5;
fn build_bvh_node(primitives: Vec<Rc<dyn Bounded>>) -> BvhNode {
    if primitives.len() <= 1 {
        return BvhNode::Leaf(BvhLeaf { primitives });
    }

    let combined =
        full_bbox(&primitives).expect("Failed to create combined bounding box for slice");

    // TODO: do for axes other than x

    let x_range = combined.x();

    let mut buckets = Vec::new();
    let bucket_size = (x_range.end - x_range.start) / (NUM_BUCKETS as f64);

    for _ in 0..NUM_BUCKETS {
        buckets.push(Bucket {
            bbox: None,
            count: 0,
        });
    }

    for prim in primitives.iter() {
        let centroid = prim.centroid();

        assert!(centroid.x() >= x_range.start);

        // Epsilon makes sure the bucket_idx is within 0 <= b < NUM_BUCKETS
        let bucket_idx = ((centroid.x() - x_range.start - f64::EPSILON) / bucket_size) as usize;
        let bucket = &mut buckets[bucket_idx];

        bucket.count += 1;

        if let Some(ref mut bbox) = bucket.bbox {
            *bbox += prim.bbox();
        } else {
            bucket.bbox = Some(prim.bbox());
        }
    }

    // TODO: find the best split by cost, partition by the best split, build bvhnodes based on split and create inner node, add terminal check for making leaf nodes

    let mut best_cost = f64::INFINITY;
    let mut best_split = 0.0;

    for split_idx in 0..(NUM_BUCKETS - 1) {
        let mut left_cnt = 0;
        let left_bbox = &buckets[..=split_idx]
            .iter()
            .filter_map(|b| {
                left_cnt += b.count;
                b.bbox.clone()
            })
            .reduce(|acc, item| acc + item)
            .expect("Unable to create left partition bounding box");

        let mut right_cnt = 0;
        let right_bbox = &buckets[(split_idx + 1)..]
            .iter()
            .filter_map(|b| {
                right_cnt += b.count;
                b.bbox.clone()
            })
            .reduce(|acc, item| acc + item)
            .expect("Unable to create left partition bounding box");

        let cost = COST_TRAVERSAL
            + (left_bbox.surface_area() * (left_cnt as f64)
                + right_bbox.surface_area() * (right_cnt as f64))
                / combined.surface_area();

        if cost < best_cost {
            best_cost = cost;
            best_split = bucket_size * ((split_idx + 1) as f64);
        }
    }

    let leaf_cost: f64 = buckets.iter().map(|b| b.count).sum::<u32>() as f64;

    if primitives.len() > MAX_PRIMS_PER_NODE || leaf_cost > best_cost {
        let (left_prims, right_prims): (Vec<_>, Vec<_>) = primitives
            .into_iter()
            .partition(|prim| prim.centroid().x() < best_split);

        let left = Rc::new(build_bvh_node(left_prims));
        let right = Rc::new(build_bvh_node(right_prims));

        BvhNode::Inner(BvhInner { left, right })
    } else {
        BvhNode::Leaf(BvhLeaf { primitives })
    }
}

enum BvhNode {
    Inner(BvhInner),
    Leaf(BvhLeaf),
}

struct BvhInner {
    left: Rc<BvhNode>,
    right: Rc<BvhNode>,
}

struct BvhLeaf {
    primitives: Vec<Rc<dyn Bounded>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_bbox() {
        let items: Vec<_> = (vec![
            BoundingBox::new(0.0..1.0, 0.0..1.0, 0.0..1.0),
            BoundingBox::new(0.0..1.0, 0.0..1.0, 0.0..1.0),
            BoundingBox::new(0.0..3.0, 0.0..1.0, 0.0..1.0),
            BoundingBox::new(-2.0..1.0, 5.0..8.0, 0.0..1.0),
        ])
        .into_iter()
        .map(|x| {
            let rc: Rc<dyn Bounded> = Rc::new(x);
            rc
        })
        .collect();

        let combined = full_bbox(&items).unwrap();

        eprintln!("{:?}", combined);

        assert_eq!(combined.x(), -2.0..3.0);
        assert_eq!(combined.y(), 0.0..8.0);
        assert_eq!(combined.z(), 0.0..1.0);
    }
}
