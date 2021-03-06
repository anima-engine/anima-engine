// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use math::Vector;

/// A `macro` useful for defining Bézier curves.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate anima_engine;
/// # use anima_engine::math::Bezier;
/// # use anima_engine::math::Vector;
/// fn main() {
///     let b = bez!(
///         (0.0, 0.0, 0.0),
///         (0.0, 0.0, 0.0),
///         (0.0, 0.0, 0.0),
///         (0.0, 0.0, 0.0)
///     );
/// }
/// ```
#[macro_export]
macro_rules! bez {
    (
        ( $x1:expr, $y1:expr, $z1:expr ),
        ( $x2:expr, $y2:expr, $z2:expr ),
        ( $x3:expr, $y3:expr, $z3:expr )
    ) => {
        Bezier::new_sqr(
            Vector::new($x1, $y1, $z1),
            Vector::new($x2, $y2, $z2),
            Vector::new($x3, $y3, $z3)
        )
    };

    (
        ( $x1:expr, $y1:expr, $z1:expr ),
        ( $x2:expr, $y2:expr, $z2:expr ),
        ( $x3:expr, $y3:expr, $z3:expr ),
        ( $x4:expr, $y4:expr, $z4:expr )
    ) => {
        Bezier::new_cub(
            Vector::new($x1, $y1, $z1),
            Vector::new($x2, $y2, $z2),
            Vector::new($x3, $y3, $z3),
            Vector::new($x4, $y4, $z4)
        )
    };
}

/// A `macro` useful for defining Bézier paths.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate anima_engine;
/// # use anima_engine::math::BezierPath;
/// # use anima_engine::math::Bezier;
/// # use anima_engine::math::Vector;
/// fn main() {
///     let p = path!(
///         bez!(
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0)
///         ),
///         bez!(
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0),
///             (0.0, 0.0, 0.0)
///         )
///     );
/// }
/// ```
#[macro_export]
macro_rules! path {
    ( $( $curve:expr ),* ) => {
        BezierPath::new(
            vec![$( $curve ),*]
        )
    }
}

/// A `struct` useful for creating square and cubic Bézier curves.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bezier {
    v1: Vector,
    v2: Vector,
    v3: Vector,
    v4: Option<Vector>
}

impl Bezier {
    /// Creates a square Bézier from `v1` to `v3` curving towards `v2`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let b = Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(2.0, 0.0, 0.0)
    /// );
    /// ```
    pub fn new_sqr(v1: Vector, v2: Vector, v3: Vector) -> Bezier {
        Bezier {
            v1: v1,
            v2: v2,
            v3: v3,
            v4: None
        }
    }

    /// Creates a cubic Bézier from `v1` to `v4` curving towards `v2` and `v3`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let b = Bezier::new_cub(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(0.0, 1.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(1.0, 0.0, 0.0)
    /// );
    /// ```
    pub fn new_cub(v1: Vector, v2: Vector, v3: Vector, v4: Vector) -> Bezier {
        Bezier {
            v1: v1,
            v2: v2,
            v3: v3,
            v4: Some(v4)
        }
    }

    /// Computes the vector on a Bézier curve correspoding to a `ratio` (between `0.0` and `1.0`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let b = Bezier::new_sqr(
    ///     Vector::new(1.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(0.0, 1.0, 0.0)
    /// );
    ///
    /// assert_eq!(b.interpolate(0.5), Vector::new(0.75, 0.75, 0.0));
    /// ```
    ///
    /// ```
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let b = Bezier::new_cub(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(0.0, 1.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(1.0, 0.0, 0.0)
    /// );
    ///
    /// assert_eq!(b.interpolate(0.5), Vector::new(0.5, 0.75, 0.0));
    /// ```
    pub fn interpolate(&self, ratio: f32) -> Vector {
        match self.v4 {
            Some(v4) => {
                self.v1 * (1.0 - ratio).powi(3) +
                self.v2 * 3.0 * (1.0 - ratio).powi(2) * ratio +
                self.v3 * 3.0 * (1.0 - ratio) * ratio.powi(2) +
                v4 * ratio.powi(3)
            },
            None => {
                self.v1 * (1.0 - ratio).powi(2) +
                self.v2 * 2.0 * (1.0 - ratio) * ratio +
                self.v3 * ratio.powi(2)
            }
        }
    }

    /// Computes the approximated length of a Bézier curve by summing the distances between `steps`
    /// uniformly distrubuted, consecutive points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// # use std::f32::consts;
    /// // approximation of radius 1.0 circle arc
    /// let b = Bezier::new_cub(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(0.0, 0.55228, 0.0),
    ///     Vector::new(0.44772, 1.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0)
    /// );
    ///
    /// const EPSILON: f32 = 0.001;
    ///
    /// assert!((b.len(20) - consts::PI / 2.0).abs() < EPSILON);
    /// ```
    pub fn len(&self, steps: i32) -> f32 {
        let (length, _) = (1..steps + 1).fold((0.0, self.v1), |(l, v), i| {
            let n = self.interpolate((i as f32) / (steps as f32));

            (l + v.dist(n), n)
        });

        length
    }
}

use mrusty::*;

mrusty_class!(Bezier, {
    def!("initialize", |mruby; args| {
        match args.len() {
            3 => {
                Bezier::new_sqr(
                    (*args[0].to_obj::<Vector>().unwrap()).clone(),
                    (*args[1].to_obj::<Vector>().unwrap()).clone(),
                    (*args[2].to_obj::<Vector>().unwrap()).clone()
                )
            }
            4 => {
                Bezier::new_cub(
                    (*args[0].to_obj::<Vector>().unwrap()).clone(),
                    (*args[1].to_obj::<Vector>().unwrap()).clone(),
                    (*args[2].to_obj::<Vector>().unwrap()).clone(),
                    (*args[3].to_obj::<Vector>().unwrap()).clone()
                )
            }
            _ => {
                return mruby.raise("ArgumentError", "wrong number of arguments")
            }
        }
    });

    def!("interpolate", |mruby, slf: Bezier, ratio: f64| {
        mruby.obj(slf.interpolate(ratio as f32))
    });

    def!("length", |mruby, slf: Bezier; args| {
        match args.len() {
            0 => mruby.float(slf.len(20) as f64),
            1 => mruby.float(slf.len(args[0].to_i32().unwrap()) as f64),
            _ => mruby.raise("ArgumentError", "wrong number of arguments")
        }
    });
});

/// A `struct` useful for creating a path of Bézier curves.
#[derive(Clone, Debug, PartialEq)]
pub struct BezierPath {
    /// `Vec<Bezier>` of curves forming the path
    pub curves: Vec<Bezier>,
    /// `Vec<f32>` containing the lengths of the `Bezier` curves with the same indices;
    /// (normalized so that they add up to `1.0`)
    pub lengths: Vec<f32>
}

impl BezierPath {
    /// Creates a Bézier path using `Bezier` curves. Curves must be connected.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::BezierPath;
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let p = BezierPath::new(vec!(Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 0.0, 0.0),
    ///     Vector::new(2.0, 0.0, 0.0)
    /// )));
    ///
    /// assert_eq!(p, BezierPath {
    ///     curves: vec!(Bezier::new_sqr(
    ///         Vector::new(0.0, 0.0, 0.0),
    ///         Vector::new(1.0, 0.0, 0.0),
    ///         Vector::new(2.0, 0.0, 0.0)
    ///     )),
    ///     lengths: vec!(1.0)
    /// });
    /// ```
    pub fn new(curves: Vec<Bezier>) -> BezierPath {
        const STEPS: i32 = 20;

        let lengths: Vec<f32> = curves.iter().map(|c| c.len(STEPS)).collect();
        let sum = lengths.iter().fold(0.0, |s, l| s + l);

        BezierPath {
            curves: curves,
            lengths: lengths.iter().map(|l| l / sum).collect()
        }
    }

    /// Computes the vector on a Bézier path correspoding to a `ratio` (between `0.0` and `1.0`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::BezierPath;
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let b1 = Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(2.0, 2.0, 0.0)
    /// );
    /// let b2 = Bezier::new_sqr(
    ///     Vector::new(2.0, 2.0, 0.0),
    ///     Vector::new(6.0, 6.0, 0.0),
    ///     Vector::new(10.0, 10.0, 0.0)
    /// );
    /// let p = BezierPath::new(vec![b1, b2]);
    ///
    /// assert_eq!(p.interpolate(0.5), Vector::new(5.0, 5.0, 0.0));
    /// assert_eq!(p.interpolate(1.2), Vector::new(12.0, 12.0, 0.0));
    /// ```
    pub fn interpolate(&self, ratio: f32) -> Vector {
        let mut sum = 0.0;

        let curve_length = self.curves.iter().zip(self.lengths.iter()).find(|&(_, l)| {
            if ratio <= sum + l {
                true
            } else {
                sum = sum + l;

                false
            }
        });

        let (curve, ratio) = match curve_length {
            Some((curve, length)) => (curve, (ratio - sum) / length),
            None                  => {
                let curve = self.curves.last();
                let length = self.lengths.last();

                match (curve, length) {
                    (Some(curve), Some(length)) => {
                        (curve, (ratio - sum + length) / length)
                    },
                    _ => panic!("Cannot interpolate an empty path.")
                }
            }
        };

        curve.interpolate(ratio)
    }

    /// Computes the approximated length of a Bézier path by summing the distances between `steps`
    /// uniformly distrubuted, consecutive points per curve.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::BezierPath;
    /// # use anima_engine::math::Bezier;
    /// # use anima_engine::math::Vector;
    /// let b1 = Bezier::new_sqr(
    ///     Vector::new(0.0, 0.0, 0.0),
    ///     Vector::new(1.0, 1.0, 0.0),
    ///     Vector::new(2.0, 2.0, 0.0)
    /// );
    /// let b2 = Bezier::new_sqr(
    ///     Vector::new(2.0, 2.0, 0.0),
    ///     Vector::new(6.0, 6.0, 0.0),
    ///     Vector::new(10.0, 10.0, 0.0)
    /// );
    /// let p = BezierPath::new(vec![b1, b2]);
    ///
    /// const EPSILON: f32 = 0.001;
    ///
    /// assert!((p.len(20) - 14.142137).abs() < EPSILON);
    /// ```
    pub fn len(&self, steps: i32) -> f32 {
        self.curves.iter().map(|curve| curve.len(steps)).fold(0.0, |s, l| s + l)
    }
}

mrusty_class!(BezierPath, {
    def!("initialize", |mruby, curves: Vec| {
        let mut beziers = Vec::with_capacity(curves.len());

        for curve in curves {
            let curve = match curve.class().to_str() {
                "Bezier" => (*curve.to_obj::<Bezier>().unwrap()).clone(),
                "Array"  => {
                    fn to_vector(mruby: MrubyType, value: &Value) -> Result<Vector, Value> {
                        match value.class().to_str() {
                            "Vector" => Ok((*value.to_obj::<Vector>().unwrap()).clone()),
                            "Array"  => {
                                let array = value.to_vec().unwrap();

                                let x = array[0].to_f64().unwrap();
                                let y = array[1].to_f64().unwrap();
                                let z = array[2].to_f64().unwrap();

                                Ok(Vector::new(x as f32, y as f32, z as f32))
                            }
                            _ => Err(mruby.raise("ArgumentError",
                                                 "Array should contain Vector or Array"))
                        }
                    }

                    let array = curve.to_vec().unwrap();

                    match array.len() {
                        3 => {
                            Bezier::new_sqr(
                                match to_vector(mruby.clone(), &array[0]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                },
                                match to_vector(mruby.clone(), &array[1]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                },
                                match to_vector(mruby.clone(), &array[2]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                }
                            )
                        }
                        4 => {
                            Bezier::new_cub(
                                match to_vector(mruby.clone(), &array[0]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                },
                                match to_vector(mruby.clone(), &array[1]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                },
                                match to_vector(mruby.clone(), &array[2]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                },
                                match to_vector(mruby.clone(), &array[3]) {
                                    Ok(v)  => v,
                                    Err(v) => {
                                        return v;
                                    }
                                }
                            )
                        }
                        _ => {
                            return mruby.raise("ArgumentError", "Array should contain 3-4 items")
                        }
                    }
                },
                _ => {
                    return mruby.raise("ArgumentError", "pass Array of Bezier or Array")
                }
            };

            beziers.push(curve);
        }

        BezierPath::new(beziers)
    });

    def!("interpolate", |mruby, slf: BezierPath, ratio: f64| {
        mruby.obj(slf.interpolate(ratio as f32))
    });

    def!("length", |mruby, slf: BezierPath; args| {
        match args.len() {
            0 => mruby.float(slf.len(20) as f64),
            1 => mruby.float(slf.len(args[0].to_i32().unwrap()) as f64),
            _ => mruby.raise("ArgumentError", "wrong number of arguments")
        }
    });
});

#[cfg(test)]
mod test_bezier {
    use mrusty::*;

    use super::Bezier;
    use super::super::Vector;

    describe!(Bezier, (Vector), "
      context 'when square arc' do
        subject { Bezier.new Vector.forward, Vector.uniform(1.0), Vector.left }

        it 'interpolates Vectors on #interpolate' do
          expect(subject.interpolate 0.5).to eql Vector.new(0.75, 0.5, 0.75)
        end

        it 'returns approximated length on #length' do
          expect(subject.length).to be_within(0.000001).of 1.950975
        end

        it 'returns approximated length on #length with custom number of steps' do
          expect(subject.length 10).to be_within(0.01).of 1.950975
        end
      end
    ");
}

#[cfg(test)]
mod test_bezier_path {
    use mrusty::*;

    use super::Bezier;
    use super::BezierPath;
    use super::super::Vector;

    describe!(BezierPath, (Bezier, Vector), "
      context 'when initialized in all possible ways' do
        subject do
          BezierPath.new [
            [
              [1.0, 1.0, 1.0],
              [1.0, 1.0, 1.0],
              Vector.uniform(1.0)
            ],
            Bezier.new(
                Vector.uniform(1.0),
                Vector.uniform(1.0),
                Vector.uniform(1.0)
            )
          ]
        end

        it 'returns approximated length on #length' do
          expect(subject.length).to be_within(0.00001).of 0.0
        end
      end

      context 'when formed of two curves' do
        subject do
          BezierPath.new [
            [
              [0.0, 0.0, 0.0],
              [1.0, 1.0, 0.0],
              [2.0, 2.0, 0.0]
            ],
            [
              [2.0, 2.0, 0.0],
              [6.0, 6.0, 0.0],
              [10.0, 10.0, 0.0]
            ]
          ]
        end

        it 'interpolates Vectors on #interpolate' do
          interpolated = subject.interpolate 0.5

          expect(interpolated.x).to be_within(0.000001).of 5.0
          expect(interpolated.y).to be_within(0.000001).of 5.0
          expect(interpolated.z).to be_within(0.000001).of 0.0
        end
      end
    ");
}
