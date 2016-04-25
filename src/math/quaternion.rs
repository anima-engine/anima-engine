// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use math::Vector;

/// A simple quaterion `struct` tailored specifically for graphics.
///
/// # Examples
///
/// ```
/// # use anima_engine::math::Quaternion;
/// # use anima_engine::math::Vector;
/// use std::f32::consts;
///
/// let q1 = Quaternion::new_rot(Vector::up(), consts::PI / 4.0);
/// let q2 = Quaternion::new_rot(Vector::up(), consts::PI / 2.0);
///
/// let q3 = q1 * q1;
///
/// const EPSILON: f32 = 0.00001;
///
/// assert!((q3.x - q2.x).abs() < EPSILON);
/// assert!((q3.y - q2.y).abs() < EPSILON);
/// assert!((q3.z - q2.z).abs() < EPSILON);
/// assert!((q3.w - q2.w).abs() < EPSILON);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    /// `f32` imaginary *i* value
    pub x: f32,
    /// `f32` imaginary *j* value
    pub y: f32,
    /// `f32` imaginary *k* value
    pub z: f32,
    /// `f32` real value
    pub w: f32
}

impl Quaternion {
    /// Creates a quaternion using 4 values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 2.0, 3.0);
    ///
    /// assert_eq!(q, Quaternion { x: 0.0, y: 1.0, z: 2.0, w: 3.0 });
    /// ```
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x: x, y: y, z: z, w: w }
    }

    /// Creates a quaternion equivalent to a rotation around a direction.
    /// The rotation is measured in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// # use anima_engine::math::Vector;
    /// # use std::f32::consts;
    /// let q1 = Quaternion::new_rot(Vector::up(), consts::PI / 2.0);
    /// let q2 = Quaternion { x: 0.0, y: 0.70710677, z: 0.0, w: 0.70710677 };
    ///
    /// const EPSILON: f32 = 0.00001;
    ///
    /// assert!((q1.x - q2.x).abs() < EPSILON);
    /// assert!((q1.y - q2.y).abs() < EPSILON);
    /// assert!((q1.z - q2.z).abs() < EPSILON);
    /// assert!((q1.w - q2.w).abs() < EPSILON);
    /// ```
    pub fn new_rot(direction: Vector, angle: f32) -> Quaternion {
        let direction = direction.norm();
        let sin = (angle / 2.0).sin();

        Quaternion {
            x: direction.x * sin,
            y: direction.y * sin,
            z: direction.z * sin,
            w: (angle / 2.0).cos()
        }
    }

    /// Creates a quaternion equivalent to the shortest rotation necessary to move
    /// the vector representing the direction `start` to the one representing `finish`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// # use anima_engine::math::Vector;
    /// let q = Quaternion::new_sph_rot(Vector::new(1.0, 1.0, 0.0), Vector::new(1.0, 1.0, 1.0));
    /// let v = Vector::new(-1.0, -1.0, 0.0);
    ///
    /// const EPSILON: f32 = 0.00001;
    ///
    /// assert!((v.rot(q) - Vector::new_unf(-0.8164966)).len() < EPSILON);
    pub fn new_sph_rot(start: Vector, finish: Vector) -> Quaternion {
        let direction = finish.cross(start);
        let angle = start.angle(finish);

        Quaternion::new_rot(direction, angle)
    }

    /// Creates an identity (0.0, 0.0, 0.0, 1.0) quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// assert_eq!(Quaternion::ident(), Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
    /// ```
    pub fn ident() -> Quaternion {
        Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    /// Computes the conjugate of a quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// let q = Quaternion::new(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(q.conj(), Quaternion { x: -1.0, y: -1.0, z: -1.0, w: 1.0 });
    /// ```
    pub fn conj(&self) -> Quaternion {
        Quaternion { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    /// Computes the inverse of a quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// let q = anima_engine::math::Quaternion::new(0.0, 1.0, 2.0, 3.0);
    ///
    /// let result = q * q.inv();
    /// let identity = anima_engine::math::Quaternion::ident();
    ///
    /// assert_eq!(result.x, identity.x);
    /// ```
    pub fn inv(&self) -> Quaternion {
        let norm = self.x.powi(2) +
                   self.y.powi(2) +
                   self.z.powi(2) +
                   self.w.powi(2);

        Quaternion {
            x: -self.x / norm,
            y: -self.y / norm,
            z: -self.z / norm,
            w: self.w / norm
        }
    }

    /// Computes the dot product between two quaternions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// let q1 = Quaternion::new(1.0, 2.0, 2.0, 1.0);
    /// let q2 = Quaternion::new(3.0, 3.0, 1.0, 1.0);
    ///
    /// assert_eq!(q1.dot(q2), 12.0);
    /// ```
    pub fn dot(&self, other: Quaternion) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }

    /// Computes the angle in radians between two quaternions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Quaternion;
    /// # use anima_engine::math::Vector;
    /// # use std::f32::consts;
    /// let q = Quaternion::new_rot(Vector::up(), consts::PI / 2.0);
    ///
    /// assert_eq!(Quaternion::ident().angle(q), consts::PI / 2.0);
    /// ```
    pub fn angle(&self, other: Quaternion) -> f32 {
        self.dot(other).acos() * 2.0
    }
}

use std::ops::Mul;

use mrusty::*;

use math::Interpolate;

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            x: other.w * self.x + other.x * self.w + other.y * self.z - other.z * self.y,
            y: other.w * self.y - other.x * self.z + other.y * self.w + other.z * self.x,
            z: other.w * self.z + other.x * self.y - other.y * self.x + other.z * self.w,
            w: other.w * self.w - other.x * self.x - other.y * self.y - other.z * self.z
        }
    }
}

impl Interpolate for Quaternion {
    fn interpolate(&self, other: Quaternion, ratio: f32) -> Quaternion {
        let cos_htheta = self.dot(other);
        let htheta = cos_htheta.acos();
        let sin_htheta = htheta.sin();

        if sin_htheta == 0.0 { panic!("Cannot interpolate between two opposing rotations."); }

        let ratio1 = ((1.0 - ratio) * htheta).sin() / sin_htheta;
        let ratio2 = (ratio * htheta).sin() / sin_htheta;

        Quaternion {
            x: self.x * ratio1 + other.x * ratio2,
            y: self.y * ratio1 + other.y * ratio2,
            z: self.z * ratio1 + other.z * ratio2,
            w: self.w * ratio1 + other.w * ratio2
        }
    }
}

mrusty_class!(Quaternion, {
    def!("initialize", |x: f64, y: f64, z: f64, w: f64| {
        Quaternion::new(x as f32, y as f32, z as f32, w as f32)
    });

    def_self!("rotation", |mruby, _slf: Value, direction: Vector, angle: f64| {
        let quaternion = Quaternion::new_rot((*direction).clone(), angle as f32);

        mruby.obj(quaternion)
    });

    def_self!("sph_rotation", |mruby, _slf: Value, start: Vector, finish: Vector| {
        let quaternion = Quaternion::new_sph_rot((*start).clone(), (*finish).clone());

        mruby.obj(quaternion)
    });

    def_self!("identity", |mruby, _slf: Value| {
        mruby.obj(Quaternion::ident())
    });

    def!("x", |mruby, slf: Quaternion| {
        mruby.float(slf.x as f64)
    });

    def!("y", |mruby, slf: Quaternion| {
        mruby.float(slf.y as f64)
    });

    def!("z", |mruby, slf: Quaternion| {
        mruby.float(slf.z as f64)
    });

    def!("w", |mruby, slf: Quaternion| {
        mruby.float(slf.w as f64)
    });

    def!("==", |mruby, slf: Quaternion, other: Quaternion| {
        let result = slf.x == other.x &&
                     slf.y == other.y &&
                     slf.z == other.z &&
                     slf.w == other.w;

        mruby.bool(result)
    });

    def!("to_s", |mruby, slf: Quaternion| {
        let string = format!("<Quaternion: @x={} @y={} @z={} @w={}>",
                             slf.x, slf.y, slf.z, slf.w);

        mruby.string(&string)
    });

    def!("*", |mruby, slf: Quaternion, other: Quaternion| {
        mruby.obj((*slf).clone() * (*other).clone())
    });

    def!("conj", |mruby, slf: Quaternion| {
        mruby.obj(slf.conj())
    });

    def!("inv", |mruby, slf: Quaternion| {
        mruby.obj(slf.inv())
    });

    def!("dot", |mruby, slf: Quaternion, other: Quaternion| {
        mruby.float(slf.dot((*other).clone()) as f64)
    });

    def!("angle", |mruby, slf: Quaternion, other: Quaternion| {
        mruby.float(slf.angle((*other).clone()) as f64)
    });

    def!("interpolate", |mruby, slf: Quaternion, other: Quaternion, ratio: f64| {
        mruby.obj(slf.interpolate((*other).clone(), ratio as f32))
    });
});

#[cfg(test)]
mod tests {
    use mrusty::*;

    use super::Quaternion;
    use super::super::Vector;

    describe!(Quaternion, (Vector), "
      context 'when roation' do
        subject { Quaternion.rotation(Vector.up, Math::PI / 2) }
        let(:second) { Quaternion.sph_rotation(Vector.forward, Vector.right) }

        it 'returns x on #x' do
          expect(subject.x).to be_within(0.001).of second.x
        end

        it 'returns y on #y' do
          expect(subject.y).to be_within(0.001).of second.y
        end

        it 'returns z on #z' do
          expect(subject.z).to be_within(0.001).of second.z
        end

        it 'returns w on #w' do
          expect(subject.w).to be_within(0.001).of second.w
        end

        it 'computes angle on #angle' do
          expect(subject.angle Quaternion.identity).to be_within(0.000001).of Math::PI / 2
        end

        it 'interpolates on #interpolate' do
          interpolated = subject.interpolate(Quaternion.rotation(Vector.up, Math::PI), 0.5)
          correct = Quaternion.rotation(Vector.up, Math::PI * 3 / 4)

          expect(interpolated.x).to be_within(0.001).of correct.x
          expect(interpolated.y).to be_within(0.001).of correct.y
          expect(interpolated.z).to be_within(0.001).of correct.z
          expect(interpolated.w).to be_within(0.001).of correct.w
        end
      end

      context 'when unit' do
        subject { Quaternion.new 1.0, 1.0, 1.0, 1.0 }

        it 'returns x on #x' do
          expect(subject.x).to eql 1.0
        end

        it 'returns y on #y' do
          expect(subject.y).to eql 1.0
        end

        it 'returns z on #z' do
          expect(subject.z).to eql 1.0
        end

        it 'returns w on #w' do
          expect(subject.w).to eql 1.0
        end

        it 'converts to String on #to_s' do
          expect(subject.to_s).to eql '<Quaternion: @x=1 @y=1 @z=1 @w=1>'
        end

        it 'returns inverse on #inv' do
          expect(subject * subject.inv).to eql Quaternion.identity
        end

        it 'returns conjugate on #conj' do
          expect(subject.conj).to eql Quaternion.new -1.0, -1.0, -1.0, 1.0
        end

        it 'computes dot product on #dot' do
          expect(subject.dot subject).to eql 4.0
        end

        it 'multiplies quaternion on #*' do
          expect(subject * Quaternion.identity).to eql subject
        end
      end
    ");
}
