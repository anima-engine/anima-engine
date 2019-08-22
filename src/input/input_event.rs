// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glium::glutin::Event;

use super::IntermediateEvent;

/// A `trait` that contains all either `Raw` or `Intermediate` input events.
#[derive(Debug)]
pub enum InputEvent {
    Raw(Event),
    Intermediate(IntermediateEvent)
}
