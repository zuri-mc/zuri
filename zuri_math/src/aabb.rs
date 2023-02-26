use std::ops::{Add, AddAssign, Sub, SubAssign};

use glam::Vec3;

/// An axis-aligned bounding box. Defines an area in the world. This area is always parallel with
/// the X, Y and Z axis and cannot be rotated.
#[derive(Clone, Debug, PartialEq)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    /// Creates a new Axis-Aligned Bounding Box. The two provided positions do not need to be a
    /// true minimum/maximum. The maximum and minimum will automatically be determined.
    pub fn new(pos1: Vec3, pos2: Vec3) -> Self {
        Self {
            min: Vec3::new(pos1.x.min(pos2.x), pos1.y.min(pos2.y), pos1.z.min(pos2.z)),
            max: Vec3::new(pos1.x.max(pos2.x), pos1.y.max(pos2.y), pos1.z.max(pos2.z)),
        }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn center(&self) -> Vec3 {
        (self.max + self.min) / 2.
    }

    /// The size of the AABB in each axis. Each component of the returned vector is the distance
    /// from the center of the box to the 'wall' on that axis.
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Check whether 2 AABB's intersect with each other. Returns false if they exactly collide but
    /// don't actually intersect.
    ///
    /// ```_
    /// The following boxes do not intersect:
    /// +---+
    /// |   |
    /// |   +-----+
    /// |   |     |
    /// +---+     |
    ///     +-----+
    ///
    /// These boxes do intersect:
    /// +---+
    /// |   |
    /// | +-|-----+
    /// | | |     |
    /// +-|-+     |
    ///   +-------+
    /// ```
    pub fn intersects_with(&self, other: &AABB) -> bool {
        ((self.min.x + self.size().x / 2.) - (other.min.x + other.size().x / 2.)).abs() * 2. < (self.size().x + other.size().x)
            && ((self.min.y + self.size().y / 2.) - (other.min.y + other.size().y / 2.)).abs() * 2. < (self.size().y + other.size().y)
            && ((self.min.z + self.size().z / 2.) - (other.min.z + other.size().z / 2.)).abs() * 2. < (self.size().z + other.size().z)
    }

    /// Returns how much two AABB's intersect with each other, or None if they do not intersect.
    ///
    /// Always returns the vector with the shortest possible distance needed to resolve the vector.
    /// Translating the AABB passed as the 'other' parameter with the returned vector will resolve
    /// the collision.
    ///
    /// ```_
    /// This example shows the rerturned vector as `x->`:
    /// +-------+
    /// |    +--|-------+
    /// |    |  |       |
    /// |    x-->       |
    /// |    |  |  other|
    /// |    +--|-------+
    /// |self   |
    /// +-------+
    ///
    /// The vector is to scale.
    /// ```
    pub fn intersect_depth(&self, other: &AABB) -> Option<Vec3> {
        if !self.intersects_with(other) {
            return None;
        }
        let dx = (self.max.x - other.min.x).min(other.max.x - self.min.x);
        let dy = (self.max.y - other.min.y).min(other.max.y - self.min.y);
        let dz = (self.max.z - other.min.z).min(other.max.z - self.min.z);

        if dx.abs() > dy.abs() && dx.abs() > dz.abs() {
            // Collision in the x-axis
            Some(Vec3::new(if self.center().x > other.center().x {
                -dx
            } else {
                dx
            }, 0., 0.))
        } else if dz.abs() > dx.abs() && dz.abs() > dy.abs() {
            // Collision in the z-axis
            Some(Vec3::new(0., 0., if self.center().z > other.center().z {
                -dz
            } else {
                dz
            }))
        } else {
            // Collision in the y-axis
            Some(Vec3::new(0., if self.center().y > other.center().y {
                -dy
            } else {
                dy
            }, 0.))
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl AddAssign<Vec3> for AABB {
    fn add_assign(&mut self, rhs: Vec3) {
        self.min += rhs;
        self.max += rhs;
    }
}

impl Sub<Vec3> for AABB {
    type Output = AABB;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            min: self.min - rhs,
            max: self.max - rhs,
        }
    }
}

impl SubAssign<Vec3> for AABB {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.min -= rhs;
        self.max -= rhs;
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;
    use crate::aabb::AABB;

    #[test]
    fn test_intersect() {
        let cases = vec![
            (
                // Two boxes that obviously intersect
                AABB::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.)),
                AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0.5, 0.5, 0.5)),
                Some(Vec3::new(0., -0.5, 0.)),
            ),
            (
                // Two boxes that obviously don't intersect
                AABB::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.)),
                AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(-0.1, -0.1, -0.1)),
                None
            ),
            (
                // Two boxes that collide only at a single point, but don't actually intersect
                AABB::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.)),
                AABB::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::new(0., 0., -0.)),
                None
            ),
            (
                // Two boxes that intersects less in the y-axis.
                AABB::new(Vec3::new(0., 0., 0.), Vec3::new(10., 10., 10.)),
                AABB::new(Vec3::new(4., -5., 4.), Vec3::new(6., 5., 6.)),
                Some(Vec3::new(0., -5., 0.)),
            ),
        ];

        let mut i = 1;
        for (this, other, intersect_depth) in &cases {
            assert_eq!(this.intersects_with(other), match intersect_depth {
                Some(_) => true,
                None => false,
            }, "test case {}/{} failed (intersects_with)", i, cases.len());

            assert_eq!(
                this.intersect_depth(other), *intersect_depth,
                "test case {}/{} failed (intersect_depth)", i, cases.len(),
            );
            i += 1;
        }
    }
}
