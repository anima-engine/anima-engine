// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::f32::consts;

/// A `struct` useful to easily compute interpolation ratios.
///
/// # Examples
///
/// ```
/// # use anima_engine::math::Interpolator;
/// # use anima_engine::math::Behavior;
/// let i = Interpolator::new(10.0, 5.0, Behavior::Linear);
///
/// assert_eq!(i.ratio(12.5), 0.5);
/// ```
///
/// ```
/// # use anima_engine::math::Interpolator;
/// # use anima_engine::math::Behavior;
/// # use anima_engine::math::Interpolate;
/// # use anima_engine::math::Vector;
/// let i = Interpolator::new(10.0, 5.0, Behavior::Linear);
/// let v1 = Vector::new_unf(0.0);
/// let v2 = Vector::new_unf(2.0);
///
/// assert_eq!(v1.interpolate(v2, i.ratio(12.5)), Vector::new_unf(1.0));
/// ```
///
/// ```
/// # use anima_engine::math::Interpolator;
/// # use anima_engine::math::Behavior;
/// # use anima_engine::math::Interpolate;
/// # use anima_engine::math::Quaternion;
/// # use anima_engine::math::Vector;
/// # use std::f32::consts;
/// let i = Interpolator::new(0.0, 2.0, Behavior::Linear);
/// let q1 = Quaternion::ident();
/// let q2 = Quaternion::new_rot(Vector::right(), consts::PI / 2.0);
///
/// let qi = q1.interpolate(q2, i.ratio(1.0));
///
/// const EPSILON: f32 = 0.00001;
///
/// assert!((q1.angle(qi) - consts::PI / 4.0).abs() < EPSILON);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interpolator {
    /// `f32` specifying the starting time of interpolation (maps to `0.0`)
    pub start: f32,
    /// `f32` specifying the duration of interpolation (`start + duration` maps to `0.0`)
    pub duration: f32,
    /// `Behavior` of the interpolation
    pub behavior: Behavior
}

/// An `enum` containing useful interpolation techniques.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Behavior {
    /// linear, *i(t) = t*
    Linear,
    /// accelerate, *i(t) = t²*
    Acc,
    /// decelerate, *i(t) = 1 - (1 - t)²*
    Dec,
    /// accelerate-decelerate, *i(t) = cos((t + 1) π) / 2 + 0.5*
    AccDec
}

impl Interpolator {
    /// Creates an interpolator by defining its starting time, duration and behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Interpolator;
    /// # use anima_engine::math::Behavior;
    /// let i = Interpolator::new(0.0, 10.0, Behavior::Linear);
    ///
    /// assert_eq!(i, Interpolator { start: 0.0, duration: 10.0, behavior: Behavior::Linear });
    /// ```
    pub fn new(start: f32, duration: f32, behavior: Behavior) -> Interpolator {
        Interpolator {
            start: start,
            duration: duration,
            behavior: behavior
        }
    }

    /// Computes the ratio (between `0.0` and `1.0`) for some given time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Interpolator;
    /// # use anima_engine::math::Behavior;
    /// let lin = Interpolator::new(0.0, 1.0, Behavior::Linear);
    /// let acc = Interpolator::new(0.0, 1.0, Behavior::Acc);
    /// let dec = Interpolator::new(0.0, 1.0, Behavior::Dec);
    /// let acd = Interpolator::new(0.0, 1.0, Behavior::AccDec);
    ///
    /// assert_eq!(lin.ratio(0.25), 0.25);
    /// assert_eq!(acc.ratio(0.25), 0.0625);
    /// assert_eq!(dec.ratio(0.25), 0.4375);
    /// assert_eq!(acd.ratio(0.25), 0.14644668);
    /// ```
    pub fn ratio(&self, time: f32) -> f32 {
        let ratio = self.convert(time);

        match self.behavior {
            Behavior::Linear => ratio,
            Behavior::Acc    => ratio.powi(2),
            Behavior::Dec    => 1.0 - (1.0 - ratio).powi(2),
            Behavior::AccDec => ((ratio + 1.0) * consts::PI).cos() / 2.0 + 0.5
        }
    }

    fn convert(&self, time: f32) -> f32 {
        (time - self.start) / self.duration
    }
}

use mrusty::*;

mrclass!(Interpolator, {
    def!("initialize", |mruby, start: f64, duration: f64, behavior: Value| {
        let behavior = match behavior.to_str().unwrap() {
            "linear" => Behavior::Linear,
            "acc"    => Behavior::Acc,
            "dec"    => Behavior::Dec,
            "accdec" => Behavior::AccDec,
            _        => {
                return mruby.raise("ArgumentError",
                                   "behavior must be one of :linear, :acc, :dec, :accdec")
            }
        };

        Interpolator::new(start as f32, duration as f32, behavior)
    });

    def!("start", |mruby, slf: Interpolator| {
        mruby.float(slf.start as f64)
    });

    def!("duration", |mruby, slf: Interpolator| {
        mruby.float(slf.duration as f64)
    });

    def!("behavior", |mruby, slf: Interpolator| {
        let behavior = match slf.behavior {
            Behavior::Linear => "linear",
            Behavior::Acc    => "acc",
            Behavior::Dec    => "dec",
            Behavior::AccDec => "accdec"
        };

        mruby.symbol(behavior)
    });

    def!("==", |mruby, slf: Interpolator, other: Interpolator| {
        let result = slf.start == other.start &&
                     slf.duration == other.duration &&
                     slf.behavior == other.behavior;

        mruby.bool(result)
    });

    def!("to_s", |mruby, slf: Interpolator| {
        let behavior = match slf.behavior {
            Behavior::Linear => "linear",
            Behavior::Acc    => "acc",
            Behavior::Dec    => "dec",
            Behavior::AccDec => "accdec"
        };

        let string = format!("<Interpolator: @start={} @duration={} @behavior=:{}>",
                             slf.start, slf.duration, behavior);

        mruby.string(&string)
    });

    def!("ratio", |mruby, slf: Interpolator, ratio: f64| {
        mruby.float(slf.ratio(ratio as f32) as f64)
    });
});

#[cfg(test)]
mod tests {
    use mrusty::*;

    use super::Interpolator;

    describe!(Interpolator, "
      context 'when linear' do
        subject { Interpolator.new 0.0, 1.0, :linear }

        it { is_expected.to eql Interpolator.new(0.0, 1.0, :linear) }

        it 'interpolates linearly on #ratio' do
          expect(subject.ratio 0.25).to eql 0.25
        end

        it 'returns start on #start' do
          expect(subject.start).to eql 0.0
        end

        it 'returns duration on #duration' do
          expect(subject.duration).to eql 1.0
        end

        it 'returns behavior on #behavior' do
          expect(subject.behavior).to eql :linear
        end

        it 'converts to String on #to_s' do
          expect(subject.to_s).to eql '<Interpolator: @start=0 @duration=1 @behavior=:linear>'
        end
      end

      context 'when accelerate' do
        subject { Interpolator.new 0.0, 1.0, :acc }

        it 'interpolates acceleratingly on #ratio' do
          expect(subject.ratio 0.25).to eql 0.0625
        end
      end

      context 'when decelerate' do
        subject { Interpolator.new 0.0, 1.0, :dec }

        it 'interpolates deceleratingly on #ratio' do
          expect(subject.ratio 0.25).to eql 0.4375
        end
      end

      context 'when accelerate-decelerate' do
        subject { Interpolator.new 0.0, 1.0, :accdec }

        it 'interpolates accelerate-deceleratingly on #ratio' do
          expect(subject.ratio 0.25).to be_within(0.000001).of 0.146446
        end
      end
    ");
}
