// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `mod` containing scripting-related helpers.

use mrusty::*;

use super::math::Bezier;
use super::math::Interpolator;
use super::math::Matrix;
use super::math::Quaternion;
use super::math::Vector;

/// A `fn` that returns a new mruby state with `require`able Anima API.
///
/// API is structured in virtual mruby files thus:
///
/// * `math`
///   * `Bezier`
///   * `Interpolator`
///   * `Matrix`
///   * `Quaternion`
///   * `Vector`
///
/// # Examples
///
/// ```
/// # use anima_engine::mrusty::*;
/// # use anima_engine::scripting;
/// let mruby = scripting::get_mruby();
///
/// mruby.run("require 'math'; Vector.uniform 0.1").unwrap();
/// ```
pub fn get_mruby() -> MrubyType {
    let mruby = Mruby::new();

    mruby.def_file::<Bezier>("math");
    mruby.def_file::<Interpolator>("math");
    mruby.def_file::<Matrix>("math");
    mruby.def_file::<Quaternion>("math");
    mruby.def_file::<Vector>("math");

    mruby
}
