use std::{fmt::Display, ops::Range, sync::Arc};

use crate::{
    geometry::{Point, Ray},
    object::{Visible, VisibleHit},
};

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

// BVH Parameters
const NUM_BUCKETS: usize = 10;
const COST_TRAVERSAL: f64 = 1.0 / 4.0;
const MAX_PRIMS_PER_NODE: usize = 4;

pub trait Bounded {
    fn bbox(&self) -> BoundingBox;
    fn surface_area(&self) -> f64;
    fn centroid(&self) -> Point<f64>;
}

pub trait Intersect {
    // TODO: Could be changed to an Option<Range<f64>> to help with optimized queries
    fn intersect(&self, r: Ray, t_range: &Range<f64>) -> Option<f64>;
}

pub trait Primitive: Bounded + Visible + Sync + Send {}

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
    // TODO: convert to a linear representation to improve data locality and speed
    root: BvhNode,
}

impl BvhTree {
    pub fn build(primitives: Vec<Arc<dyn Primitive>>) -> Self {
        let root = BvhNode::build(primitives);

        Self { root }
    }

    pub fn print(&self) {
        self.root.print(0);
    }
}

impl Visible for BvhTree {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        self.root.bounce(r, t_range)
    }
}

fn containing_bbox(items: &[Arc<dyn Primitive>]) -> BoundingBox {
    items
        .iter()
        .map(|p| p.bbox())
        .reduce(|acc, item| acc + item)
        .unwrap_or(BoundingBox::empty())
}

struct Bucket {
    count: u32,
    bbox: Option<BoundingBox>,
}

struct PrimitiveInfo {
    primitive: Arc<dyn Primitive>,
    centroid: Point<f64>,
    bbox: BoundingBox,
}

enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn of(&self, p: &Point<f64>) -> f64 {
        match self {
            Axis::X => p.x(),
            Axis::Y => p.y(),
            Axis::Z => p.z(),
        }
    }
}
impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
        };

        write!(f, "{}", name)
    }
}

struct Split {
    axis: Axis,
    threshold: f64,
    cost: f64,
}

impl Split {
    fn partition(
        &self,
        items: Vec<PrimitiveInfo>,
    ) -> (Vec<Arc<dyn Primitive>>, Vec<Arc<dyn Primitive>>) {
        let (left, right): (Vec<_>, Vec<_>) = items
            .into_iter()
            .partition(|item| self.axis.of(&item.centroid) < self.threshold);

        let left = left
            .into_iter()
            .map(|prim_info| prim_info.primitive)
            .collect();
        let right = right
            .into_iter()
            .map(|prim_info| prim_info.primitive)
            .collect();

        (left, right)
    }
}

// const COST_TRAVERSAL: f64 = f64::INFINITY;
// const MAX_PRIMS_PER_NODE: usize = 5;

impl Visible for BvhNode {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        self.intersect(r, t_range)?;

        match self {
            BvhNode::Inner(inner) => {
                let (left_t, right_t) = (
                    inner.left.intersect(r, t_range),
                    inner.right.intersect(r, t_range),
                );

                match (left_t, right_t) {
                    (Some(left_t), Some(right_t)) => {
                        let (first, first_t, second, second_t) = if left_t < right_t {
                            (&inner.left, left_t, &inner.right, right_t)
                        } else {
                            (&inner.right, right_t, &inner.left, left_t)
                        };

                        let closest_hit = first.bounce(r, t_range);

                        match closest_hit {
                            Some(hit) => {
                                let mut best_hit = hit;

                                if best_hit.t >= second_t {
                                    if let Some(second_hit) = second.bounce(r, t_range) {
                                        if best_hit.t > second_hit.t {
                                            best_hit = second_hit;
                                        }
                                    }
                                }

                                Some(best_hit)
                            }
                            None => second.bounce(r, t_range),
                        }
                    }
                    (Some(left_t), None) => inner.left.bounce(r, t_range),
                    (None, Some(right_t)) => inner.right.bounce(r, t_range),
                    _ => None,
                }
            }
            BvhNode::Leaf(leaf) => {
                let mut closest_t = t_range.end;

                leaf.primitives
                    .iter()
                    .filter_map(|x| {
                        let hit = x.bounce(r, &(t_range.start..closest_t));

                        if let Some(ref record) = hit {
                            closest_t = f64::min(closest_t, record.t);
                        }

                        hit
                    })
                    .reduce(|acc, hit| if acc.t > hit.t { hit } else { acc })
            }
        }
    }
}

impl BvhNode {
    fn print(&self, offset: usize) {
        let gap = "\t".repeat(offset);
        print!("{}", gap);

        match self {
            BvhNode::Inner(inner) => {
                println!(
                    "Inner (split on {} at {})",
                    inner.split.axis, inner.split.threshold
                );

                inner.left.print(offset + 1);
                inner.right.print(offset + 1);
            }
            BvhNode::Leaf(leaf) => println!("Leaf ({})", leaf.primitives.len()),
        }
    }

    // FIXME: fails on equal centroids

    fn find_best_split(primitives: &Vec<PrimitiveInfo>, axis: Axis) -> Option<Split> {
        let centroids = primitives.iter().map(|p| axis.of(&p.centroid));
        let centroid_bounds = centroids
            .map(|cent| cent..cent)
            .reduce(|acc, item| acc.start.min(item.start)..acc.end.max(item.end))?;

        let mut buckets: Vec<_> = (0..NUM_BUCKETS)
            .map(|_| Bucket {
                bbox: None,
                count: 0,
            })
            .collect();

        let bucket_size = (centroid_bounds.end - centroid_bounds.start) / (NUM_BUCKETS as f64);

        // assign each primitive to a bucket
        for prim in primitives.iter() {
            let coord = axis.of(&prim.centroid);

            let mut b = ((coord - centroid_bounds.start) / bucket_size).floor() as usize;
            if b == buckets.len() {
                b -= 1;
            }

            buckets[b].bbox = match &buckets[b].bbox {
                Some(bbox) => Some(bbox.clone() + prim.bbox.clone()),
                None => Some(prim.bbox.clone()),
            };
            buckets[b].count += 1;
        }

        // consider all splits
        let mut best_threshold = None;
        let mut best_cost = f64::INFINITY;

        for split_idx in 0..(buckets.len() - 1) {
            let left_buckets = &buckets[..=split_idx];
            let right_buckets = &buckets[(split_idx + 1)..];

            let left_bbox = left_buckets
                .iter()
                .filter_map(|b| b.bbox.clone())
                .reduce(|acc, bbox| acc + bbox);
            let right_bbox = right_buckets
                .iter()
                .filter_map(|b| b.bbox.clone())
                .reduce(|acc, bbox| acc + bbox);

            let left_count: u32 = left_buckets.iter().map(|b| b.count).sum();
            let right_count: u32 = right_buckets.iter().map(|b| b.count).sum();

            match (left_bbox, right_bbox) {
                (Some(left_bbox), Some(right_bbox)) => {
                    let left_sa = left_bbox.surface_area();
                    let right_sa = right_bbox.surface_area();

                    let split_cost = COST_TRAVERSAL
                        + (left_count as f64 * left_sa)
                        + (right_count as f64 * right_sa);

                    if split_cost < best_cost {
                        best_cost = split_cost;
                        best_threshold =
                            Some(centroid_bounds.start + ((split_idx + 1) as f64 * bucket_size));
                    }
                }
                _ => continue,
            };
        }

        Some(Split {
            axis,
            threshold: best_threshold?,
            cost: best_cost,
        })
    }

    fn build(primitives: Vec<Arc<dyn Primitive>>) -> Self {
        let containing_bbox = containing_bbox(&primitives);

        // precompute the computations needed for primitives
        let primitive_info: Vec<_> = primitives
            .into_iter()
            .map(|prim| PrimitiveInfo {
                bbox: prim.bbox(),
                centroid: prim.centroid(),
                primitive: prim,
            })
            .collect();

        let split_candidates = vec![
            BvhNode::find_best_split(&primitive_info, Axis::X),
            BvhNode::find_best_split(&primitive_info, Axis::Y),
            BvhNode::find_best_split(&primitive_info, Axis::Z),
        ];

        // choose the axis with the lowest cost, or all might be none
        let best_split = split_candidates
            .into_iter()
            .filter_map(|s| s)
            .reduce(|acc, split| if acc.cost > split.cost { split } else { acc });

        let leaf_cost = primitive_info.len() as f64;
        match best_split {
            Some(split) if split.cost < leaf_cost || primitive_info.len() > MAX_PRIMS_PER_NODE => {
                let (left, right) = split.partition(primitive_info);

                let left_child = BvhNode::build(left);
                let right_child = BvhNode::build(right);

                BvhNode::Inner(BvhInner {
                    bbox: containing_bbox,
                    split,
                    left: Arc::new(left_child),
                    right: Arc::new(right_child),
                })
            }
            _ => BvhNode::Leaf(BvhLeaf {
                bbox: containing_bbox,
                primitives: primitive_info.into_iter().map(|p| p.primitive).collect(),
            }),
        }
    }
}

enum BvhNode {
    Inner(BvhInner),
    Leaf(BvhLeaf),
}

impl Intersect for BvhNode {
    fn intersect(&self, r: Ray, t_range: &Range<f64>) -> Option<f64> {
        match self {
            BvhNode::Inner(inner) => inner.bbox.intersect(r, t_range),
            BvhNode::Leaf(leaf) => leaf.bbox.intersect(r, t_range),
        }
    }
}

struct BvhInner {
    bbox: BoundingBox,
    split: Split,
    left: Arc<BvhNode>,
    right: Arc<BvhNode>,
}

struct BvhLeaf {
    bbox: BoundingBox,
    primitives: Vec<Arc<dyn Primitive>>,
}
