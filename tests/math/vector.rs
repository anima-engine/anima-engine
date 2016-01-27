// Anima Engine  Copyright (C) 2015  Dragoș Tiselice
// This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
// This is free software, and you are welcome to redistribute it
// under certain conditions; type `show c' for details.
//
// The hypothetical commands `show w' and `show c' should show the appropriate
// parts of the General Public License.  Of course, your program's commands
// might be different; for a GUI interface, you would use an "about box".
//
// You should also get your employer (if you work as a programmer) or school,
// if any, to sign a "copyright disclaimer" for the program, if necessary.
// For more information on this, and how to apply and follow the GNU GPL, see
// <http://www.gnu.org/licenses/>.

extern crate anima;

#[test]
fn test_add() {
    let v1 = anima::math::Vector::new_unf(1.0);
    let v2 = anima::math::Vector::new_unf(2.0);

    assert_eq!(v1 + v2, anima::math::Vector { x: 3.0, y: 3.0, z: 3.0 });
}

#[test]
fn test_mul() {
    let v1 = anima::math::Vector::new_unf(1.0);
    let v2 = anima::math::Vector::new_unf(2.0);

    assert_eq!(v1 * v2, anima::math::Vector { x: 2.0, y: 2.0, z: 2.0 });
}
