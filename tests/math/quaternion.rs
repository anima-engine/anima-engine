// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate anima;

#[test]
fn test_mul() {
    let q1 = anima::math::Quaternion::new(0.0, 1.0, 2.0, 3.0);
    let q2 = anima::math::Quaternion::new(3.0, 2.0, 1.0, 0.0);

    assert_eq!(q1 * q2, anima::math::Quaternion { x: 12.0, y: 0.0, z: 6.0, w: -4.0 });
}

#[test]
fn test_neg() {
    let q = anima::math::Quaternion::new(0.0, 1.0, 2.0, 3.0);

    let result = q * -q;
    let identity = anima::math::Quaternion::ident();

    assert!((result.x - identity.x).abs() < 0.0001);
    assert!((result.y - identity.y).abs() < 0.0001);
    assert!((result.z - identity.z).abs() < 0.0001);
    assert!((result.w - identity.w).abs() < 0.0001);
}
