// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

use time::Duration;

use mrusty::*;

use super::game::Game;
use super::super::scripting;


/// A `struct` used to run games from mruby directly.
///
/// Make sure you point to an mruby file with a `Game` `Class` defined which implements a method
/// `update(dt)`, where `dt` is a `Float` representing the time since the last frame.
///
/// # Examples
///
/// ```no-run
/// let game = MrubyGame::new(Path::new("game.rb"));
///
/// GameLoop::new(game).run();
/// ```
pub struct MrubyGame {
    pub mruby: MrubyType,
    pub game: Value
}

impl MrubyGame {
    /// Creates a new `MrubyGame` from an mruby script.
    ///
    /// Make sure you point to an mruby file with a `Game` `Class` defined which implements a
    /// method `update(dt)`, where `dt` is a `Float` representing the time since the last frame.
    ///
    /// # Examples
    ///
    /// ```no-run
    /// let game = MrubyGame::new(Path::new("game.rb"));
    ///
    /// GameLoop::new(game).run();
    /// ```
    pub fn new(script: &Path) -> MrubyGame {
        let mruby = scripting::get_mruby();

        mruby.execute(script).unwrap();

        let game = mruby.run("Game.new")
                        .unwrap_or_else(|_| { panic!("Game class must be defined in mruby") });

        MrubyGame {
            mruby: mruby,
            game: game
        }
    }
}

impl Game for MrubyGame {
    fn update(&self, dt: Duration) -> bool {
        let seconds = dt.num_seconds() as f64;
        let nanoseconds = match dt.num_nanoseconds() {
            Some(nanoseconds) => nanoseconds as f64 / 1.0e+9,
            None              => 0f64
        };

        let dt = self.mruby.float(seconds + nanoseconds);

        self.game.call("update", vec![dt]).unwrap().to_bool().unwrap()
    }
}
