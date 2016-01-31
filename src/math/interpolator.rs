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

use std::f32::consts;

/// A `struct` useful to easily compute interpolation ratios.
///
/// # Examples
///
/// ```
/// # use anima::math::Interpolator;
/// # use anima::math::Behavior;
/// let i = Interpolator::new(10.0, 5.0, Behavior::Linear);
///
/// assert_eq!(i.ratio(12.5), 0.5);
/// ```
///
/// ```
/// # use anima::math::Interpolator;
/// # use anima::math::Behavior;
/// # use anima::math::Interpolate;
/// # use anima::math::Vector;
/// let i = Interpolator::new(10.0, 5.0, Behavior::Linear);
/// let v1 = Vector::new_unf(0.0);
/// let v2 = Vector::new_unf(2.0);
///
/// assert_eq!(v1.interpolate(v2, i.ratio(12.5)), Vector::new_unf(1.0));
/// ```
///
/// ```
/// # use anima::math::Interpolator;
/// # use anima::math::Behavior;
/// # use anima::math::Interpolate;
/// # use anima::math::Quaternion;
/// # use anima::math::Vector;
/// # use std::f32::consts;
/// let i = Interpolator::new(0.0, 2.0, Behavior::Linear);
/// let q1 = Quaternion::ident();
/// let q2 = Quaternion::new_rot(Vector::right(), consts::PI / 2.0);
///
/// let qi = q1.interpolate(q2, i.ratio(1.0));
///
/// const EPSILON: f32 = 0.00001;
///
/// assert!((q1.angle(qi) - consts::PI / 4.0).abs() < EPSILON);
/// ```
#[derive(Debug, PartialEq)]
pub struct Interpolator {
    /// `f32` specifying the starting time of interpolation (maps to `0.0`)
    pub start: f32,
    /// `f32` specifying the duration of interpolation (`start + duration` maps to `0.0`)
    pub duration: f32,
    /// `Behavior` of the interpolation
    pub behavior: Behavior
}

/// An `enum` containing useful interpolation techniques.
#[derive(Debug, Eq, PartialEq)]
pub enum Behavior {
    /// linear, *i(t) = t*
    Linear,
    /// accelerate, *i(t) = t²*
    Acc,
    /// decelerate, *i(t) = 1 - (1 - t)²*
    Dec,
    /// accelerate-decelerate, *i(t) = cos((t + 1) π) / 2 + 0.5*
    AccDec
}

impl Interpolator {
    /// Creates an interpolator by it's starting time, duration and behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::Interpolator;
    /// # use anima::math::Behavior;
    /// let i = Interpolator::new(0.0, 10.0, Behavior::Linear);
    ///
    /// assert_eq!(i, Interpolator { start: 0.0, duration: 10.0, behavior: Behavior::Linear });
    /// ```
    pub fn new(start: f32, duration: f32, behavior: Behavior) -> Interpolator {
        Interpolator {
            start: start,
            duration: duration,
            behavior: behavior
        }
    }

    /// Computes the ratio (between `0.0` and `1.0`) for some given time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima::math::Interpolator;
    /// # use anima::math::Behavior;
    /// let lin = Interpolator::new(0.0, 1.0, Behavior::Linear);
    /// let acc = Interpolator::new(0.0, 1.0, Behavior::Acc);
    /// let dec = Interpolator::new(0.0, 1.0, Behavior::Dec);
    /// let acd = Interpolator::new(0.0, 1.0, Behavior::AccDec);
    ///
    /// assert_eq!(lin.ratio(0.25), 0.25);
    /// assert_eq!(acc.ratio(0.25), 0.0625);
    /// assert_eq!(dec.ratio(0.25), 0.4375);
    /// assert_eq!(acd.ratio(0.25), 0.14644668);
    /// ```
    pub fn ratio(&self, time: f32) -> f32 {
        let ratio = self.convert(time);

        match self.behavior {
            Behavior::Linear => ratio,
            Behavior::Acc    => ratio.powi(2),
            Behavior::Dec    => 1.0 - (1.0 - ratio).powi(2),
            Behavior::AccDec => ((ratio + 1.0) * consts::PI).cos() / 2.0 + 0.5
        }
    }

    fn convert(&self, time: f32) -> f32 {
        (time - self.start) / self.duration
    }
}
