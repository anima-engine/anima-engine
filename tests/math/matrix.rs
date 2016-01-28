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
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate anima;

use std::f32::consts;

type Matrix = anima::math::Matrix;
type Vector = anima::math::Vector;
type Quaternion = anima::math::Quaternion;

#[test]
fn test_mul() {
    let mut a1 = [0.0; 16];
    let mut a2 = [0.0; 16];

    for i in (0..16) {
        a1[i] = (i as f32) + 1.0;
        a2[i] = 16.0 - (i as f32);
    }

    let m1 = Matrix::new(a1);
    let m2 = Matrix::new(a2);

    assert_eq!((m1 * m2).array, [
        386.0, 444.0, 502.0, 560.0,
        274.0, 316.0, 358.0, 400.0,
        162.0, 188.0, 214.0, 240.0,
         50.0,  60.0,  70.0,  80.0
    ]);
}

#[test]
fn test_linearity() {
    let v = Vector::new(1.0, 1.0, 0.0);

    let m = Matrix::ident()
            .rot_around(Quaternion::new_rot(Vector::right(), consts::PI), Vector::new(0.0, 1.0, 0.0))
            .rot(Quaternion::new_rot(Vector::forward(), consts::PI / 4.0))
            .scale(Vector::new(1.0, 1.0 / 2.0f32.sqrt(), 1.0))
            .trans(Vector::new(1.0, -1.0, 0.0));

    let v = m * v;
    let v = Vector::new(v.x.round(), v.y.round(), v.z.round());

    assert_eq!(v, Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn test_inv() {
    let m = Matrix::ident().rot(Quaternion::new_rot(Vector::forward(), consts::PI / 4.0));

    assert_eq!(m * m.inv(), Matrix::ident());
}

#[test]
#[should_panic]
fn test_det_0_panic() {
    let m = Matrix::new([0.0; 16]);
    
    m.inv();
}
