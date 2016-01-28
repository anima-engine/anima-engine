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

type Vector = anima::math::Vector;

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
