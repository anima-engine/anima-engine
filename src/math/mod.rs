// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `mod` containing linear math constructs useful in graphics.

mod vector;
mod quaternion;
mod matrix;

mod interpolate;
mod interpolator;

mod bezier;

pub use self::vector::Vector;
pub use self::quaternion::Quaternion;
pub use self::matrix::Matrix;

pub use self::interpolate::Interpolate;
pub use self::interpolator::Interpolator;
pub use self::interpolator::Behavior;

pub use self::bezier::Bezier;
pub use self::bezier::BezierPath;
