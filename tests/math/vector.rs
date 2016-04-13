// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate anima_engine;

use self::anima_engine::math::Vector;

#[test]
fn test_add() {
    let v1 = Vector::new_unf(1.0);
    let v2 = Vector::new_unf(2.0);

    assert_eq!(v1 + v2, Vector::new(3.0, 3.0, 3.0));
}

#[test]
fn test_mul() {
    let v1 = Vector::new_unf(1.0);
    let v2 = Vector::new_unf(2.0);

    assert_eq!(v1 * v2, Vector::new(2.0, 2.0, 2.0));
}

#[test]
fn test_scalar_mul() {
    let v = Vector::new_unf(1.0);

    assert_eq!(v * 2.0, Vector::new(2.0, 2.0, 2.0));
    assert_eq!(2.0 * v, Vector::new(2.0, 2.0, 2.0));
}

#[test]
fn test_neg() {
    let v = Vector::new_unf(1.0);

    assert_eq!(-v, Vector::new(-1.0, -1.0, -1.0));
}
