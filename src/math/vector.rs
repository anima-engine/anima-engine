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

/// A simple vector `struct` tailored specifically for graphics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    /// `f32` x coordinate value
    pub x: f32,
    /// `f32` y coordinate value
    pub y: f32,
    /// `f32` y coordinate value
    pub z: f32
}

impl Vector {
    /// Creates a vector using 3 values.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(0.0, 1.0, 2.0);
    ///
    /// assert_eq!(v, Vector { x: 0.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    /// Creates a vector using an array.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new_arr([0.0, 1.0, 2.0]);
    ///
    /// assert_eq!(v, Vector { x: 0.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn new_arr(array: [f32; 3]) -> Vector {
        Vector { x: array[0], y: array[1], z: array[2] }
    }

    /// Creates a uniform vector using 1 value.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new_unf(1.0);
    ///
    /// assert_eq!(v, Vector { x: 1.0, y: 1.0, z: 1.0 });
    /// ```
    pub fn new_unf(v: f32) -> Vector {
        Vector { x: v, y: v, z: v }
    }

    /// Computes the length of a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    ///
    /// assert_eq!(v.length(), 3.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

use std::ops::Add;
use std::ops::Mul;
use std::cmp::Ordering;

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}
