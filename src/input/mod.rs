// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `mod` useful for controling input.

mod input_event;
mod intermediate_event;
mod intermediate;

pub use glium::glutin::{Event, MouseButton};

pub use self::input_event::InputEvent;
pub use self::intermediate_event::IntermediateEvent;
pub use self::intermediate::Button;
pub use self::intermediate::Cursor;
pub use self::intermediate::Intermediate;
