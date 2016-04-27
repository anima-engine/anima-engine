// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glium::glutin::MouseButton;

/// A `trait` that contains all possible intermediate events generated with
/// `Intermediate`-implementing generators.
#[derive(Debug)]
pub enum IntermediateEvent {
    ButtonPressed(u32),
    ButtonCanceled(u32),
    ButtonReleased(u32),
    CursorPressed(i32, i32, MouseButton),
    CursorReleased(i32, i32, MouseButton)
}
