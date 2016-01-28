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
}


use std::ops::Mul;

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
