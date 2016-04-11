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

        let game = match mruby.run("Game.new") {
            Ok(game) => game,
            Err(_)   => panic!("Game class must be defined in mruby")
        };

        MrubyGame {
            mruby: mruby,
            game: game
        }
    }
}

impl Game for MrubyGame {
    fn update(&self, dt: Duration) {
        let seconds = dt.num_seconds() as f64;
        let nanoseconds = match dt.num_nanoseconds() {
            Some(nanoseconds) => nanoseconds as f64 / 1.0e+9,
            None              => 0f64
        };

        let dt = self.mruby.float(seconds + nanoseconds);

        self.game.call("update", vec![dt]).unwrap();
    }
}
