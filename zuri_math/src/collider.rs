use glam::Vec3;

use crate::aabb::AABB;

/// A collection of Axis-Align Bounding Boxes that make up a collider. The resulting area is equal
/// to the union of all the AABBs used to create the collider.
/// ```_
/// Using the following two boxes:
/// +---+
/// |B1 |
/// | +-|-----+
/// | | |     |
/// +-|-+  B2 |
///   +-------+
///
/// Will create a collider that spans the following area:
/// +---+
/// |   |
/// |   +-----+
/// |         |
/// +-+       |
///   +-------+
/// ```
#[derive(Clone, Debug, Default)]
pub struct Collider {
    outer: Option<AABB>,
    boxes: Vec<AABB>,
}

impl Collider {
    pub fn new(boxes: Vec<AABB>) -> Self {
        Self {
            outer: calc_outer_box(&boxes),
            boxes,
        }
    }

    pub fn empty() -> Self {
        Self {
            outer: None,
            boxes: Vec::new(),
        }
    }

    /// Return the minimum and maximum point of every AABB in the collider combined. Returns None if
    /// the collider is empty.
    /// ```_
    /// If we have a collider made up of the following two boxes:
    /// +---+
    /// |B1 |
    /// | +-|-----+
    /// | | |     |
    /// +-|-+  B2 |
    ///   +-------+
    ///
    /// The AABB returned by this function will be:
    ///            max
    /// +---------x
    /// |         |
    /// |         |
    /// |         |
    /// |         |
    /// x---------+
    ///  min
    /// ```
    pub fn outer_bounds(&self) -> &Option<AABB> {
        &self.outer
    }

    /// Returns all the AABBs that the collider consists of.
    pub fn boxes(&self) -> &Vec<AABB> {
        &self.boxes
    }

    /// Checks if two colliders intersect with each other. This is done by checking if any pair of
    /// colliders from this box and the `other` box intersect with each other.
    ///
    /// This check is done in two phases:
    ///
    /// **Broad Phase**\
    /// Checks if the outer bounds for both colliders exist. If they both exist and they intersect,
    /// proceed to the next phase. If either or both colliders are empty (and so the outer bonds are
    /// equal to None), false is returned.
    ///
    /// **Narrow Phase**\
    /// Checks every possible combination of AABBs from &self and AABBs from &other. If any of these
    /// intersect, true is returned. Otherwise, the colliders do not intersect and so false is
    /// returned.
    ///
    pub fn intersects_with(&self, other: &Collider) -> bool {
        if self.outer.is_none() || other.outer.is_none() {
            return false;
        }
        if !self
            .outer
            .as_ref()
            .unwrap()
            .intersects_with(other.outer.as_ref().unwrap())
        {
            return false;
        }

        for b1 in &self.boxes {
            for b2 in &other.boxes {
                if b1.intersects_with(b2) {
                    return true;
                }
            }
        }
        false
    }

    /// Returns how much two colliders intersect if they do intersect.
    /// This is calculated by going over every possibly intersection pair between both colliders
    /// and taking the maximum intersection_depth for each axis.
    pub fn intersect_depth(&self, other: &Collider) -> Option<Vec3> {
        if self.outer.is_none() || other.outer.is_none() {
            return None;
        }
        if !self
            .outer
            .as_ref()
            .unwrap()
            .intersects_with(other.outer.as_ref().unwrap())
        {
            return None;
        }

        let mut depth = Vec3::ZERO;
        for b1 in &self.boxes {
            for b2 in &self.boxes {
                let new_depth;
                if let Some(d) = b1.intersect_depth(b2) {
                    new_depth = d;
                } else {
                    continue;
                }

                if new_depth.x.abs() > depth.x.abs() {
                    depth.x = new_depth.x;
                }
                if new_depth.y.abs() > depth.y.abs() {
                    depth.y = new_depth.y;
                }
                if new_depth.z.abs() > depth.z.abs() {
                    depth.z = new_depth.z;
                }
            }
        }
        if depth.length_squared() == 0. {
            return None;
        }
        Some(depth)
    }
}

fn calc_outer_box(boxes: &Vec<AABB>) -> Option<AABB> {
    if boxes.is_empty() {
        return None;
    }
    let mut min = boxes[0].min();
    let mut max = boxes[1].max();
    for b in boxes {
        if b.min().x < min.x {
            min.x = b.min().x
        }
        if b.min().y < min.y {
            min.y = b.min().y
        }
        if b.max().x > min.x {
            max.x = b.max().x
        }
        if b.max().y > min.y {
            max.y = b.max().y
        }
    }
    Some(AABB::new(min, max))
}
