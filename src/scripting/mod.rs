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
