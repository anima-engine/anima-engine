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

/// A `trait` to be implemented by `struct`ures that need to handle 2-way
/// interpolation. (not necessarily linear)
///
/// # Examples
/// ```
/// # use anima::math::Interpolate;
/// struct Object {
///     height: f32
/// }
///
/// impl Interpolate for Object {
///     fn interpolate(&self, other: Object, ratio: f32) -> Object {
///         Object {
///             height: (1.0 - ratio) * self.height + ratio * other.height
///         }
///     }
/// }
/// ```
pub trait Interpolate {
    fn interpolate(&self, other: Self, ratio: f32) -> Self;
}
