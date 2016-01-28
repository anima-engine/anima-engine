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

use math::Vector;
use math::Quaternion;

/// A simple matrix `struct` tailored specifically for graphics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    /// `f32` array containing values; columns incremented first
    pub array: [f32; 16]
}

impl Matrix {
    /// Creates a matrix using a length 16 array. (columns incremented first)
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Matrix;
    /// let m = Matrix::new([1.0; 16]);
    ///
    /// assert_eq!(m, Matrix { array: [1.0; 16] });
    /// ```
    pub fn new(array: [f32; 16]) -> Matrix {
        Matrix { array: array }
    }

    /// Creates an identity (1.0 on primary diagonal) matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Matrix;
    /// let m = Matrix::new([2.0; 16]);
    ///
    /// assert_eq!(m * Matrix::ident(), Matrix { array: [2.0; 16] });
    /// ```
    pub fn ident() -> Matrix {
        let mut array = [0.0; 16];

        array[0]  = 1.0;
        array[5]  = 1.0;
        array[10] = 1.0;
        array[15] = 1.0;

        Matrix { array: array }
    }

    /// Translates a matrix according to the scale represented by a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Matrix;
    /// # use anima::math::Vector;
    /// let v = Vector::new_unf(1.0);
    /// let m = Matrix::ident().trans(Vector::new(1.0, 0.0, 1.0));
    ///
    /// assert_eq!(m * v, Vector { x: 2.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn trans(&self, vector: Vector) -> Matrix {
        let mut r = self.array;
        let v = vector;

        r[12] += r[0] * v.x + r[4] * v.y + r[8]  * v.z;
        r[13] += r[1] * v.x + r[5] * v.y + r[9]  * v.z;
        r[14] += r[2] * v.x + r[6] * v.y + r[10] * v.z;
        r[15] += r[3] * v.x + r[7] * v.y + r[11] * v.z;

        Matrix { array: r }
    }

    /// Scales a matrix according to the scale represented by a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Matrix;
    /// # use anima::math::Vector;
    /// let v = Vector::new_unf(2.0);
    /// let m = Matrix::ident().scale(Vector::new(2.0, 3.0, 4.0));
    ///
    /// assert_eq!(m * v, Vector { x: 4.0, y: 6.0, z: 8.0 });
    /// ```
    pub fn scale(&self, vector: Vector) -> Matrix {
        let mut r = self.array;
        let v = vector;

        r[0]  *= v.x;
        r[1]  *= v.x;
        r[2]  *= v.x;
        r[3]  *= v.x;
        r[4]  *= v.y;
        r[5]  *= v.y;
        r[6]  *= v.y;
        r[7]  *= v.y;
        r[8]  *= v.z;
        r[9]  *= v.z;
        r[10] *= v.z;
        r[11] *= v.z;

        Matrix { array: r }
    }

    /// Rotates a matrix according to the rotation represented by a quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Matrix;
    /// # use anima::math::Vector;
    /// # use anima::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let v = Vector::new(1.0, 0.0, 0.0);
    ///
    /// assert_eq!(Matrix::ident().rot(q) * v, Vector { x: -1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn rot(self, quaternion: Quaternion) -> Matrix {
        let q = quaternion;

        self * Matrix {
            array: [
                1.0 - 2.0 * (q.y.powi(2) + q.z.powi(2)),
                2.0 * (q.x * q.y + q.z * q.w),
                2.0 * (q.x * q.z - q.y * q.w),
                0.0,
                2.0 * (q.x * q.y - q.z * q.w),
                1.0 - 2.0 * (q.x.powi(2) + q.z.powi(2)),
                2.0 * (q.y * q.z + q.x * q.w),
                0.0,
                2.0 * (q.x * q.z + q.y * q.w),
                2.0 * (q.y * q.z - q.x * q.w),
                1.0 - 2.0 * (q.x.powi(2) + q.y.powi(2)),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0
            ]
        }
    }
}

use std::ops::Mul;

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        let l = self.array;
        let r = [vector.x, vector.y, vector.z, 1.0];

        let result = [
            l[0] * r[0]  + l[4] * r[1]  + l[8]  * r[2]  + l[12]  * r[3],
            l[1] * r[0]  + l[5] * r[1]  + l[9]  * r[2]  + l[13]  * r[3],
            l[2] * r[0]  + l[6] * r[1]  + l[10] * r[2]  + l[14]  * r[3],
            l[3] * r[0]  + l[7] * r[1]  + l[11] * r[2]  + l[15]  * r[3]
        ];

        Vector {
            x: result[0] / result[3],
            y: result[1] / result[3],
            z: result[2] / result[3]
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let l = self.array;
        let r = other.array;

        Matrix {
            array: [
                l[0] * r[0]  + l[4] * r[1]  + l[8]  * r[2]  + l[12]  * r[3],
                l[1] * r[0]  + l[5] * r[1]  + l[9]  * r[2]  + l[13]  * r[3],
                l[2] * r[0]  + l[6] * r[1]  + l[10] * r[2]  + l[14]  * r[3],
                l[3] * r[0]  + l[7] * r[1]  + l[11] * r[2]  + l[15]  * r[3],
                l[0] * r[4]  + l[4] * r[5]  + l[8]  * r[6]  + l[12]  * r[7],
                l[1] * r[4]  + l[5] * r[5]  + l[9]  * r[6]  + l[13]  * r[7],
                l[2] * r[4]  + l[6] * r[5]  + l[10] * r[6]  + l[14]  * r[7],
                l[3] * r[4]  + l[7] * r[5]  + l[11] * r[6]  + l[15]  * r[7],
                l[0] * r[8]  + l[4] * r[9]  + l[8]  * r[10] + l[12]  * r[11],
                l[1] * r[8]  + l[5] * r[9]  + l[9]  * r[10] + l[13]  * r[11],
                l[2] * r[8]  + l[6] * r[9]  + l[10] * r[10] + l[14]  * r[11],
                l[3] * r[8]  + l[7] * r[9]  + l[11] * r[10] + l[15]  * r[11],
                l[0] * r[12] + l[4] * r[13] + l[8]  * r[14] + l[12]  * r[15],
                l[1] * r[12] + l[5] * r[13] + l[9]  * r[14] + l[13]  * r[15],
                l[2] * r[12] + l[6] * r[13] + l[10] * r[14] + l[14]  * r[15],
                l[3] * r[12] + l[7] * r[13] + l[11] * r[14] + l[15]  * r[15]
            ]
        }
    }
}
