// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate anima_engine;

use self::anima_engine::math::Quaternion;

#[test]
fn test_mul() {
    let q1 = Quaternion::new(0.0, 1.0, 2.0, 3.0);
    let q2 = Quaternion::new(3.0, 2.0, 1.0, 0.0);

    assert_eq!(q1 * q2, Quaternion::new(12.0, 0.0, 6.0, -4.0));
}
