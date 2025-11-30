/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::Vec2;
use rand::Rng;
use std::{
    f32::consts::{PI, TAU},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle {
    pub radians: f32,
}

impl Angle {
    pub const ZERO: Self = Self { radians: 0.0 };
    pub const FULL: Self = Self { radians: TAU };
    pub const HALF: Self = Self { radians: PI };

    #[inline]
    pub fn new(radians: f32) -> Self {
        Self { radians }
    }

    pub fn new_random(rng: &mut impl Rng, range: Range<f32>) -> Self {
        Self {
            radians: rng.random_range(range),
        }
    }

    #[inline]
    pub fn from_vector(vec: Vec2) -> Self {
        Self {
            radians: vec.y.atan2(vec.x),
        }
    }

    #[inline]
    pub fn to_vector(self) -> Vec2 {
        let (sin, cos): (f32, f32) = self.radians.sin_cos();
        Vec2::new(cos, sin)
    }

    #[inline]
    pub fn normalize(&mut self) {
        self.radians = (self.radians + PI).rem_euclid(TAU) - PI;
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let mut angle: Self = self;
        angle.normalize();
        angle
    }

    #[inline]
    pub fn sin(self) -> f32 {
        self.radians.sin()
    }

    #[inline]
    pub fn cos(self) -> f32 {
        self.radians.cos()
    }

    #[inline]
    pub fn tan(self) -> f32 {
        self.radians.tan()
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self {
            radians: self.radians.abs(),
        }
    }

    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        let diff: f32 = (to.radians - self.radians + PI).rem_euclid(TAU) - PI;
        let t_clamped: f32 = t.clamp(0.0, 1.0);
        Self {
            radians: self.radians + diff * t_clamped,
        }
    }
}

impl From<f32> for Angle {
    #[inline]
    fn from(radians: f32) -> Self {
        Self { radians }
    }
}

impl From<Angle> for f32 {
    #[inline]
    fn from(angle: Angle) -> Self {
        angle.radians
    }
}

impl Add for Angle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            radians: self.radians + rhs.radians,
        }
    }
}

impl AddAssign for Angle {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.radians += rhs.radians;
    }
}

impl Sub for Angle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            radians: self.radians - rhs.radians,
        }
    }
}

impl SubAssign for Angle {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.radians -= rhs.radians;
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            radians: self.radians * scalar,
        }
    }
}

impl MulAssign<f32> for Angle {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.radians *= scalar;
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            radians: self.radians / scalar,
        }
    }
}

impl DivAssign<f32> for Angle {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.radians /= scalar;
    }
}

impl Neg for Angle {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            radians: -self.radians,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Angle;
    use crate::Vec2;
    use rand::{rngs::StdRng, SeedableRng};
    use std::f32::consts::{PI, TAU};

    #[test]
    fn angle_ops() {
        let mut angle: Angle = Angle::new(PI);

        assert_eq!(angle + Angle::new(PI), Angle::new(TAU));

        assert_eq!(angle * 0.5, Angle::new(PI / 2.0));

        angle += Angle::new(PI);
        assert_eq!(angle, Angle::new(TAU));
    }

    #[test]
    fn angle_normalization() {
        let angle: Angle = Angle::new(3.0 * PI);
        let normalized: Angle = angle.normalized();

        assert!((normalized.radians.abs() - PI).abs() < 1e-6);
    }

    #[test]
    fn angle_vectors() {
        let angle: Angle = Angle::new(PI / 2.0);
        let vec: Vec2 = angle.to_vector();

        assert!(vec.x.abs() < 1e-6);
        assert!((vec.y - 1.0).abs() < 1e-6);

        let from_vec: Angle = Angle::from_vector(Vec2::new(0.0, 1.0));
        assert!((from_vec.radians - PI / 2.0).abs() < 1e-6);
    }

    #[test]
    fn angle_lerp_shortest_path() {
        let start: Angle = Angle::new(0.1);
        let end: Angle = Angle::new(TAU - 0.1);

        let mid: Angle = start.lerp(end, 0.5);

        assert!(mid.radians.abs() < 1e-6);
    }

    #[test]
    fn angle_random() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let angle: Angle = Angle::new_random(&mut rng, 0.0f32..PI);

        assert!(angle.radians >= 0.0 && angle.radians < PI);
    }
}
