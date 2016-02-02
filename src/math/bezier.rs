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
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use math::Vector;

/// A `macro` useful for defining Bézier curves.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate anima;
/// # use anima::math::Bezier;
/// # use anima::math::Vector;
/// fn main() {
///     let b = bez!(
///         (0.0, 0.0, 0.0),
///         (0.0, 0.0, 0.0),
///         (0.0, 0.0, 0.0),
///         (0.0, 0.0, 0.0)
///     );
/// }
/// ```
#[macro_export]
macro_rules! bez {
    (
        ( $x1:expr, $y1:expr, $z1:expr ),
        ( $x2:expr, $y2:expr, $z2:expr ),
        ( $x3:expr, $y3:expr, $z3:expr )
    ) => {
        Bezier::new_sqr(
            Vector::new($x1, $y1, $z1),
            Vector::new($x2, $y2, $z2),
            Vector::new($x3, $y3, $z3)
        )
    };

    (
        ( $x1:expr, $y1:expr, $z1:expr ),
        ( $x2:expr, $y2:expr, $z2:expr ),
        ( $x3:expr, $y3:expr, $z3:expr ),
        ( $x4:expr, $y4:expr, $z4:expr )
    ) => {
        Bezier::new_cub(
            Vector::new($x1, $y1, $z1),
            Vector::new($x2, $y2, $z2),
            Vector::new($x3, $y3, $z3),
            Vector::new($x4, $y4, $z4)
        )
    };
}

/// A `macro` useful for defining Bézier paths.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate anima;
/// # use anima::math::BezierPath;
/// # use anima::math::Bezier;
/// # use anima::math::Vector;
/// fn main() {
///     let p = path!(
///         bez!(
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0)
///         ),
///         bez!(
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0)
///         )
///     );
/// }
/// ```
#[macro_export]
macro_rules! path {
    ( $( $curve:expr ),* ) => {
        BezierPath::new(
            vec![$( $curve ),*]
        )
    }
}

/// A `struct` useful for creating square and cubic Bézier curves.
#[derive(Clone, Copy, Debug, PartialEq)]
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

    /// Computes the vector on a Bézier curve correspoding to a `ratio` (between `0.0` and `1.0`).
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

    /// Computes the approximated length of a Bézier curve by summing the distances between `steps`
    /// uniformly distrubuted, consecutive points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// # use std::f32::consts;
    /// // approximation of radius 1.0 circle arc
    /// let b = Bezier::new_cub(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(0.0, 0.55228, 0.0),
    ///     Vector::new(0.44772, 1.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0)
    /// );
    ///
    /// const EPSILON: f32 = 0.001;
    ///
    /// assert!((b.length(20) - consts::PI / 2.0).abs() < EPSILON);
    /// ```
    pub fn length(&self, steps: i32) -> f32 {
        let (length, _) = (1..steps + 1).fold((0.0, self.v1), |(l, v), i| {
            let n = self.interpolate((i as f32) / (steps as f32));

            (l + v.dist(n), n)
        });

        length
    }
}

/// A `struct` useful for creating a path of Bézier curves.
#[derive(Clone, Debug, PartialEq)]
pub struct BezierPath {
    /// `Vec<Bezier>` of curves forming the path
    pub curves: Vec<Bezier>,
    /// `Vec<f32>` containing the lengths of the `Bezier` curves with the same indices;
    /// (normalized so that they add up to `1.0`)
    pub lengths: Vec<f32>
}

impl BezierPath {
    /// Creates a Bézier path using `Bezier` curves. Curves must be connected.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::BezierPath;
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// let p = BezierPath::new(vec!(Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 0.0, 0.0),
    ///     Vector::new(2.0, 0.0, 0.0)
    /// )));
    ///
    /// assert_eq!(p, BezierPath {
    ///     curves: vec!(Bezier::new_sqr(
    ///         Vector::new(0.0, 0.0, 0.0),
    ///         Vector::new(1.0, 0.0, 0.0),
    ///         Vector::new(2.0, 0.0, 0.0)
    ///     )),
    ///     lengths: vec!(1.0)
    /// });
    /// ```
    pub fn new(curves: Vec<Bezier>) -> BezierPath {
        const STEPS: i32 = 20;

        let lengths: Vec<f32> = curves.iter().map(|c| c.length(STEPS)).collect();
        let sum = lengths.iter().fold(0.0, |s, l| s + l);

        BezierPath {
            curves: curves,
            lengths: lengths.iter().map(|l| l / sum).collect()
        }
    }

    /// Computes the vector on a Bézier path correspoding to a `ratio` (between `0.0` and `1.0`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::BezierPath;
    /// # use anima::math::Bezier;
    /// # use anima::math::Vector;
    /// let b1 = Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(2.0, 2.0, 0.0)
    /// );
    /// let b2 = Bezier::new_sqr(
    ///     Vector::new(2.0, 2.0, 0.0),
    ///     Vector::new(6.0, 6.0, 0.0),
    ///     Vector::new(10.0, 10.0, 0.0)
    /// );
    /// let p = BezierPath::new(vec![b1, b2]);
    ///
    /// assert_eq!(p.interpolate(0.5), Vector::new(5.0, 5.0, 0.0));
    /// assert_eq!(p.interpolate(1.2), Vector::new(12.0, 12.0, 0.0));
    /// ```
    pub fn interpolate(&self, ratio: f32) -> Vector {
        let mut sum = 0.0;

        let curve_length = self.curves.iter().zip(self.lengths.iter()).find(|&(_, l)| {
            if ratio <= sum + l {
                true
            } else {
                sum = sum + l;

                false
            }
        });

        let (curve, ratio) = match curve_length {
            Some((curve, length)) => (curve, (ratio - sum) / length),
            None                  => {
                let curve = self.curves.last();
                let length = self.lengths.last();

                match (curve, length) {
                    (Some(curve), Some(length)) => {
                        (curve, (ratio - sum + length) / length)
                    },
                    _ => panic!("Cannot interpolate an empty path.")
                }
            }
        };

        curve.interpolate(ratio)
    }
}
