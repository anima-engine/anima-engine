// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `mod` containing intermediate `InputEvent` generators.

use super::InputEvent;

mod button;
mod cursor;

pub use self::button::Button;
pub use self::cursor::Cursor;

/// A `trait` that processes `InputEvent` which would normally create `IntermediateEvent`s from
/// `Raw` `InputEvent`s.
pub trait Intermediate {
    fn process(self, input: Vec<InputEvent>) -> Vec<InputEvent>;
}
