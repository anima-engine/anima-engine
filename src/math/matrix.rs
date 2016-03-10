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
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use math::{Quaternion, Vector};

/// A simple matrix `struct` tailored specifically for graphics.
///
/// # Examples
///
/// ```
/// # use anima_engine::math::Matrix;
/// # use anima_engine::math::Vector;
/// let m = Matrix::ident().scale(Vector::new_unf(2.0)).trans(Vector::one());
///
/// assert_eq!(m.array[0], 2.0);
/// assert_eq!(m.array[12], 1.0);
///
/// let inv = m.inv();
///
/// assert_eq!(m * inv, Matrix::ident());
/// assert_eq!(inv * m, Matrix::ident());
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    /// `[f32; 16]` containing values; columns incremented first
    pub array: [f32; 16]
}

impl Matrix {
    /// Creates a matrix using a length 16 array. (columns incremented first)
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
    /// let m = Matrix::new([1.0; 16]);
    ///
    /// assert_eq!(m, Matrix { array: [1.0; 16] });
    /// ```
    pub fn new(array: [f32; 16]) -> Matrix {
        Matrix { array: array }
    }

    /// Creates an identity (1.0 on primary diagonal) matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
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
    /// The translation is applied to the left. (`t * m`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new_unf(1.0);
    /// let m = Matrix::ident().trans(Vector::new(1.0, 0.0, 1.0));
    ///
    /// assert_eq!(m * v, Vector { x: 2.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn trans(&self, vector: Vector) -> Matrix {
        let m = self.array;
        let v = vector;

        Matrix {
            array: [
                m[0]  + m[3]  * v.x,
                m[1]  + m[3]  * v.y,
                m[2]  + m[3]  * v.z,
                m[3],
                m[4]  + m[7]  * v.x,
                m[5]  + m[7]  * v.y,
                m[6]  + m[7]  * v.z,
                m[7],
                m[8]  + m[11] * v.x,
                m[9]  + m[11] * v.y,
                m[10] + m[11] * v.z,
                m[11],
                m[12] + m[15] * v.x,
                m[13] + m[15] * v.y,
                m[14] + m[15] * v.z,
                m[15]
            ]
        }
    }

    /// Scales a matrix according to the scale represented by a vector.
    /// The scaling is applied to the left. (`s * m`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new_unf(2.0);
    /// let m = Matrix::ident().scale(Vector::new(2.0, 3.0, 4.0));
    ///
    /// assert_eq!(m * v, Vector { x: 4.0, y: 6.0, z: 8.0 });
    /// ```
    pub fn scale(&self, vector: Vector) -> Matrix {
        let m = self.array;
        let v = vector;

        Matrix {
            array: [
                m[0]  * v.x,
                m[1]  * v.y,
                m[2]  * v.z,
                m[3],
                m[4]  * v.x,
                m[5]  * v.y,
                m[6]  * v.z,
                m[7],
                m[8]  * v.x,
                m[9]  * v.y,
                m[10] * v.z,
                m[11],
                m[12] * v.x,
                m[13] * v.y,
                m[14] * v.z,
                m[15]
            ]
        }
    }

    /// Rotates a matrix according to the rotation represented by a quaternion.
    /// The rotation is applied to the left. (`r * m`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
    /// # use anima_engine::math::Vector;
    /// # use anima_engine::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let v = Vector::new(1.0, 0.0, 0.0);
    ///
    /// assert_eq!(Matrix::ident().rot(q) * v, Vector { x: -1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn rot(self, quaternion: Quaternion) -> Matrix {
        let q = quaternion;

        let m = Matrix {
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
        };

        m * self
    }

    /// Rotates a matrix according to the rotation represented by the quaternion around a point.
    /// The rotation is applied to the left. (`r * m`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
    /// # use anima_engine::math::Vector;
    /// # use anima_engine::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let v = Vector::new(1.0, 0.0, 0.0);
    /// let p = Vector::new(2.0, 0.0, 0.0);
    ///
    /// assert_eq!(Matrix::ident().rot_around(q, p) * v, Vector { x: 3.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn rot_around(&self, quaternion: Quaternion, point: Vector) -> Matrix {
        self.trans(-point).rot(quaternion).trans(point)
    }

    /// Inverts a matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Matrix;
    /// assert_eq!(Matrix::ident().inv(), Matrix::ident());
    /// ```
    pub fn inv(&self) -> Matrix {
        let m = self.array;

        let s0 = m[0] * m[5]  - m[1] * m[4];
        let s1 = m[0] * m[9]  - m[1] * m[8];
        let s2 = m[0] * m[13] - m[1] * m[12];
        let s3 = m[4] * m[9]  - m[5] * m[8];
        let s4 = m[4] * m[13] - m[5] * m[12];
        let s5 = m[8] * m[13] - m[9] * m[12];

        let c5 = m[10] * m[15] - m[11] * m[14];
        let c4 = m[6]  * m[15] - m[7]  * m[14];
        let c3 = m[6]  * m[11] - m[7]  * m[10];
        let c2 = m[2]  * m[15] - m[3]  * m[14];
        let c1 = m[2]  * m[11] - m[3]  * m[10];
        let c0 = m[2]  * m[7]  - m[3]  * m[6];

        let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;

        if det == 0.0 { panic!("Matrix {:?} is not invertable.", m); }

        let inv_det = det.recip();

        Matrix {
            array: [
                ( m[5] * c5 - m[9]  * c4 + m[13] * c3) * inv_det,
                (-m[1] * c5 + m[9]  * c2 - m[13] * c1) * inv_det,
                ( m[1] * c4 - m[5]  * c2 + m[13] * c0) * inv_det,
                (-m[1] * c3 + m[5]  * c1 - m[9]  * c0) * inv_det,
                (-m[4] * c5 + m[8]  * c4 - m[12] * c3) * inv_det,
                ( m[0] * c5 - m[8]  * c2 + m[12] * c1) * inv_det,
                (-m[0] * c4 + m[4]  * c2 - m[12] * c0) * inv_det,
                ( m[0] * c3 - m[4]  * c1 + m[8]  * c0) * inv_det,
                ( m[7] * s5 - m[11] * s4 + m[15] * s3) * inv_det,
                (-m[3] * s5 + m[11] * s2 - m[15] * s1) * inv_det,
                ( m[3] * s4 - m[7]  * s2 + m[15] * s0) * inv_det,
                (-m[3] * s3 + m[7]  * s1 - m[11] * s0) * inv_det,
                (-m[6] * s5 + m[10] * s4 - m[14] * s3) * inv_det,
                ( m[2] * s5 - m[10] * s2 + m[14] * s1) * inv_det,
                (-m[2] * s4 + m[6]  * s2 - m[14] * s0) * inv_det,
                ( m[2] * s3 - m[6]  * s1 + m[10] * s0) * inv_det
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
