// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use math::Vector;

/// A simple quaterion `struct` tailored specifically for graphics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    /// `f32` imaginary *i* value
    pub x: f32,
    /// `f32` imaginary *j* value
    pub y: f32,
    /// `f32` imaginary *k* value
    pub z: f32,
    /// `f32` real value
    pub w: f32
}

impl Quaternion {
    /// Creates a quaternion using 4 values.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 2.0, 3.0);
    ///
    /// assert_eq!(q, Quaternion { x: 0.0, y: 1.0, z: 2.0, w: 3.0 });
    /// ```
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x: x, y: y, z: z, w: w }
    }

    /// Creates a quaternion equivalent to a rotation around a direction.
    /// The rotation is measured in radians.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Quaternion;
    /// # use anima::math::Vector;
    /// # use std::f32::consts;
    /// let q = Quaternion::new_rot(&Vector::up(), consts::PI / 2.0);
    ///
    /// assert_eq!(q, Quaternion { x: 0.0, y: 0.70710677, z: 0.0, w: 0.70710677 });
    /// ```
    pub fn new_rot(direction: &Vector, angle: f32) -> Quaternion {
        let direction = direction.norm();
        let sin = (angle / 2.0).sin();

        Quaternion {
            x: direction.x * sin,
            y: direction.y * sin,
            z: direction.z * sin,
            w: (angle / 2.0).cos()
        }
    }

    /// Creates an identity (0.0, 0.0, 0.0, 1.0) quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Quaternion;
    /// assert_eq!(Quaternion::ident(), Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
    /// ```
    pub fn ident() -> Quaternion {
        Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    /// Computes the conjugate of a quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Quaternion;
    /// let q = Quaternion::new(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(q.conj(), Quaternion { x: -1.0, y: -1.0, z: -1.0, w: 1.0 });
    /// ```
    pub fn conj(&self) -> Quaternion {
        Quaternion { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    /// Computes the inverse of a quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Quaternion;
    /// let q = anima::math::Quaternion::new(0.0, 1.0, 2.0, 3.0);
    ///
    /// let result = q * q.inv();
    /// let identity = anima::math::Quaternion::ident();
    ///
    /// assert_eq!(result.x, identity.x);
    /// ```
    pub fn inv(&self) -> Quaternion {
        let norm = self.x.powi(2) +
                   self.y.powi(2) +
                   self.z.powi(2) +
                   self.w.powi(2);

        Quaternion {
            x: -self.x / norm,
            y: -self.y / norm,
            z: -self.z / norm,
            w: self.w / norm
        }
    }
}

use std::ops::Mul;

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: other.w * self.x + other.x * self.w + other.y * self.z - other.z * self.y,
            y: other.w * self.y - other.x * self.z + other.y * self.w + other.z * self.x,
            z: other.w * self.z + other.x * self.y - other.y * self.x + other.z * self.w,
            w: other.w * self.w - other.x * self.x - other.y * self.y - other.z * self.z
        }
    }
}
