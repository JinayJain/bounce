use std::{ops::Range, sync::Arc};

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

    fn hit_node(&self, node: &BvhNode, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        match node {
            BvhNode::Inner(inner) => {
                let left_intersect = inner.left.intersect(r, t_range);
                let right_intersect = inner.right.intersect(r, t_range);

                match left_intersect {
                    Some(left_t) => match right_intersect {
                        Some(right_t) => {
                            let (first, first_t, second, second_t) = if left_t < right_t {
                                (&inner.left, left_t, &inner.right, right_t)
                            } else {
                                (&inner.right, right_t, &inner.left, left_t)
                            };

                            let mut best_hit = self.hit_node(&first, r, &(first_t..t_range.end));
                            let first_t = best_hit.as_ref().and_then(|hit| Some(hit.t));

                            if first_t.is_none() || first_t.unwrap() > second_t {
                                let second_hit =
                                    self.hit_node(&second, r, &(second_t..t_range.end));

                                let second_t = second_hit.as_ref().and_then(|hit| Some(hit.t));

                                if let Some(new_t) = second_t {
                                    if first_t.is_none() || first_t.unwrap() > new_t {
                                        best_hit = second_hit;
                                    }
                                }
                            }

                            best_hit
                        }
                        None => self.hit_node(&inner.left, r, &(left_t..t_range.end)),
                    },
                    None => match right_intersect {
                        Some(right_t) => self.hit_node(&inner.right, r, &(right_t..t_range.end)),
                        None => None,
                    },
                }
            }
            BvhNode::Leaf(leaf) => {
                // (visible) hit all primitives in the leaf
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

impl Visible for BvhTree {
    fn bounce(&self, r: Ray, t_range: &Range<f64>) -> Option<VisibleHit> {
        self.hit_node(&self.root, r, t_range)
    }
}

#[derive(Debug)]
struct Bucket {
    count: u32,
    bbox: Option<BoundingBox>,
}

fn full_bbox(items: &[Arc<dyn Primitive>]) -> Option<BoundingBox> {
    items
        .iter()
        .map(|p| p.bbox())
        .reduce(|acc, item| acc + item)
}

fn get_bucket(value: f64, bucket_size: f64, buckets_start: f64, num_buckets: usize) -> usize {
    let idx = ((value - buckets_start) / bucket_size) as usize;

    idx.max(0).min(num_buckets - 1)
}

const NUM_BUCKETS: usize = 8;
const COST_TRAVERSAL: f64 = 1.0 / 8.0;
const MAX_PRIMS_PER_NODE: usize = 5;

impl BvhNode {
    // FIXME: fails on empty lists
    // FIXME: fails on equal centroids

    fn build(primitives: Vec<Arc<dyn Primitive>>) -> Self {
        let full_bbox =
            full_bbox(&primitives).expect("Failed to create combined bounding box for slice");

        dbg!(primitives.len());

        if primitives.len() == 1 {
            return BvhNode::Leaf(BvhLeaf {
                bbox: full_bbox,
                primitives,
            });
        }

        // TODO: do for axes other than x

        let x_range = full_bbox.x();

        let mut buckets = Vec::new();
        let bucket_size = (x_range.end - x_range.start) / (NUM_BUCKETS as f64);

        for _ in 0..NUM_BUCKETS {
            buckets.push(Bucket {
                bbox: None,
                count: 0,
            });
        }

        eprintln!("made {} buckets of size {}", NUM_BUCKETS, bucket_size);
        eprintln!("range {:?}", &x_range);

        for prim in primitives.iter() {
            let centroid = prim.centroid();

            assert!(centroid.x() >= x_range.start);
            assert!(centroid.x() <= x_range.end);

            // Epsilon makes sure the bucket_idx is within 0 <= b < NUM_BUCKETS
            println!("{}", centroid.x());
            let bucket_idx = get_bucket(centroid.x(), bucket_size, x_range.start, NUM_BUCKETS);
            let bucket = &mut buckets[bucket_idx];
            eprintln!("assigned to bucket {}", bucket_idx);

            bucket.count += 1;

            if let Some(ref mut bbox) = bucket.bbox {
                *bbox += prim.bbox();
            } else {
                bucket.bbox = Some(prim.bbox());
            }
        }

        let mut best_cost = f64::INFINITY;
        let mut best_split = None;

        for split_idx in 0..(NUM_BUCKETS - 1) {
            let mut left_cnt = 0;
            let left_bbox = &buckets[..=split_idx]
                .iter()
                .filter_map(|b| {
                    left_cnt += b.count;
                    b.bbox.clone()
                })
                .reduce(|acc, item| acc + item);

            let mut right_cnt = 0;
            let right_bbox = &buckets[(split_idx + 1)..]
                .iter()
                .filter_map(|b| {
                    right_cnt += b.count;
                    b.bbox.clone()
                })
                .reduce(|acc, item| acc + item);

            dbg!(left_cnt, right_cnt);

            if left_cnt == 0 || right_cnt == 0 {
                continue;
            }

            let left_bbox = left_bbox.as_ref().unwrap();
            let right_bbox = right_bbox.as_ref().unwrap();

            let cost = COST_TRAVERSAL
                + (left_bbox.surface_area() * (left_cnt as f64)
                    + right_bbox.surface_area() * (right_cnt as f64))
                    / full_bbox.surface_area();

            if cost < best_cost {
                dbg!(cost, best_cost);
                best_cost = cost;
                best_split = Some(bucket_size * ((split_idx + 1) as f64) + x_range.start);
            }
        }

        let leaf_cost: f64 = buckets.iter().map(|b| b.count).sum::<u32>() as f64;

        if best_split.is_some() && (primitives.len() > MAX_PRIMS_PER_NODE || leaf_cost > best_cost)
        {
            // println!("best split: {}", best_split);

            let (left_prims, right_prims): (Vec<_>, Vec<_>) = primitives
                .into_iter()
                .partition(|prim| prim.centroid().x() < best_split.unwrap());

            // dbg!(left_prims.len());
            // dbg!(right_prims.len());

            let left = Arc::new(BvhNode::build(left_prims));
            let right = Arc::new(BvhNode::build(right_prims));

            BvhNode::Inner(BvhInner {
                bbox: full_bbox,
                left,
                right,
            })
        } else {
            BvhNode::Leaf(BvhLeaf {
                bbox: full_bbox,
                primitives,
            })
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
    left: Arc<BvhNode>,
    right: Arc<BvhNode>,
}

struct BvhLeaf {
    bbox: BoundingBox,
    primitives: Vec<Arc<dyn Primitive>>,
}

#[cfg(test)]
mod tests {
    use super::get_bucket;

    #[test]
    fn bucket_selection() {
        let bucket_size = 1.0;
        let buckets_start = -3.0;
        let num_buckets = 4;

        assert_eq!(
            get_bucket(buckets_start, bucket_size, buckets_start, num_buckets),
            0
        );

        assert_eq!(get_bucket(-2.6, bucket_size, buckets_start, num_buckets), 0);
        assert_eq!(get_bucket(-1.3, bucket_size, buckets_start, num_buckets), 1);
        assert_eq!(
            get_bucket(-0.35, bucket_size, buckets_start, num_buckets),
            2
        );
        assert_eq!(
            get_bucket(0.439, bucket_size, buckets_start, num_buckets),
            3
        );

        assert_eq!(get_bucket(1.0, bucket_size, buckets_start, num_buckets), 3);
    }
}
