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

    /// Creates a (0.0, 0.0, 0.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::zero(), Vector { x: 0.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Creates a (1.0, 1.0, 1.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::one(), Vector { x: 1.0, y: 1.0, z: 1.0 });
    /// ```
    pub fn one() -> Vector {
        Vector { x: 1.0, y: 1.0, z: 1.0 }
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

    /// Computes the normalized version of a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    /// let n = v.normalize();
    ///
    /// assert_eq!(n.length(), 1.0); // Keep precision in mind when comparing floats.
    /// ```
    pub fn normalize(&self) -> Vector {
        let length = self.length();

        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length
        }
    }

    /// Computes the dot product between two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 2.0);
    /// let v2 = Vector::new(3.0, 3.0, 1.0);
    ///
    /// assert_eq!(v1.dot(v2), 11.0);
    /// ```
    pub fn dot(&self, other: Vector) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    /// Computes the cross product between two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 2.0);
    /// let v2 = Vector::new(3.0, 3.0, 1.0);
    ///
    /// assert_eq!(v1.cross(v2), Vector { x: -4.0, y: 5.0, z: -3.0 });
    /// ```
    pub fn cross(&self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    /// Converts the vector to an array of homogeneous coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(v.to_array(), [1.0, 2.0, 3.0, 1.0]);
    /// ```
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, 1.0]
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

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}
