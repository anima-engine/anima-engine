// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A `mod` containing game running constructs.

mod game;
mod game_loop;
mod mruby_game;

pub use self::game::Game;
pub use self::game_loop::GameLoop;
pub use self::mruby_game::MrubyGame;
