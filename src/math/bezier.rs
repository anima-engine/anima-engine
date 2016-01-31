// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use math::Vector;

/// A `struct` useful for creating square and cubic Bézier curves.
pub struct Bezier {
    v1: Vector,
    v2: Vector,
    v3: Vector,
    v4: Option<Vector>
}

impl Bezier {
    /// Creates a square Bézier from `v1` to `v3` curving towards `v2`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// let b = Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(2.0, 0.0, 0.0)
    /// );
    /// ```
    pub fn new_sqr(v1: Vector, v2: Vector, v3: Vector) -> Bezier {
        Bezier {
            v1: v1,
            v2: v2,
            v3: v3,
            v4: None
        }
    }

    /// Creates a cubic Bézier from `v1` to `v4` curving towards `v2` and `v3`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// let b = Bezier::new_cub(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(0.0, 1.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(1.0, 0.0, 0.0)
    /// );
    /// ```
    pub fn new_cub(v1: Vector, v2: Vector, v3: Vector, v4: Vector) -> Bezier {
        Bezier {
            v1: v1,
            v2: v2,
            v3: v3,
            v4: Some(v4)
        }
    }

    /// Computes the vector on a Bézier curve correspoding to a ratio (between `0.0` and `1.0`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// let b = Bezier::new_sqr(
    ///     Vector::new(1.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(0.0, 1.0, 0.0)
    /// );
    ///
    /// assert_eq!(b.interpolate(0.5), Vector::new(0.75, 0.75, 0.0));
    /// ```
    ///
    /// ```
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// let b = Bezier::new_cub(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(0.0, 1.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(1.0, 0.0, 0.0)
    /// );
    ///
    /// assert_eq!(b.interpolate(0.5), Vector::new(0.5, 0.75, 0.0));
    /// ```
    pub fn interpolate(&self, ratio: f32) -> Vector {
        match self.v4 {
            Some(v4) => {
                self.v1 * (1.0 - ratio).powi(3) +
                self.v2 * 3.0 * (1.0 - ratio).powi(2) * ratio +
                self.v3 * 3.0 * (1.0 - ratio) * ratio.powi(2) +
                v4 * ratio.powi(3)
            },
            None => {
                self.v1 * (1.0 - ratio).powi(2) +
                self.v2 * 2.0 * (1.0 - ratio) * ratio +
                self.v3 * ratio.powi(2)
            }
        }
    }
}
