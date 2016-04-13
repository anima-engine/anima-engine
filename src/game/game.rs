// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use time::Duration;

/// A `trait` runnable within a `GameLoop`. `dt` is the `Duration` since last frame. `update`
/// should return the boolean value of whether the game should continue.
///
/// # Examples
///
/// ```
/// # use anima_engine::time::Duration;
/// # use anima_engine::game::Game;
/// # use anima_engine::game::GameLoop;
/// pub struct MyGame;
///
/// impl Game for MyGame {
///     fn update(&self, dt: Duration) -> bool {
///         // Update game state.
///         // Return `false` when game needs to stop.
///         false
///     }
/// }
///
/// GameLoop::new(MyGame);
/// ```
pub trait Game {
    fn update(&self, dt: Duration) -> bool;
}
