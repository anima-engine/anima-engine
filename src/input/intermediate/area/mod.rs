// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `mod` containing intermediate `InputEvent` generators that work on specific areas of the
//! screen.

mod selectable_area;

pub use self::selectable_area::SelectableArea;
pub use self::selectable_area::SpecialSelect;
