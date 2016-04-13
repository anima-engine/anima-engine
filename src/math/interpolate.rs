// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// A `trait` to be implemented by `struct`ures that need to handle 2-way
/// interpolation. (not necessarily linear)
///
/// # Examples
/// ```
/// # use anima_engine::math::Interpolate;
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
