// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::game::Game;

use std::time::Instant;

/// A `struct` that helps you create a very simple game loop.
///
/// # Examples
///
/// ```
/// # use std::time::Duration;
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
/// GameLoop::new(MyGame).run();
/// ```
pub struct GameLoop<T: Game> {
    pub game: T
}

impl<T: Game> GameLoop<T> {
    /// Creates a `GameLoop`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
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
    pub fn new(game: T) -> GameLoop<T> {
        GameLoop { game: game }
    }

    /// Runs `GameLoop`'s `Game` in a loop while feeding the time between frames to the `Game`'s
    /// `update`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::time::Duration;
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
    /// GameLoop::new(MyGame).run();
    /// ```
    pub fn run(&self) {
        let mut last = Instant::now();

        while self.game.update(last.elapsed()) {
            last = Instant::now();
        }
    }
}
