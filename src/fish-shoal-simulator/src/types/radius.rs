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

use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Radius {
    pub value: f32,
}

impl Radius {
    pub const ZERO: Self = Self { value: 0.0 };
    pub const ONE: Self = Self { value: 1.0 };

    #[inline]
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn new_random(rng: &mut impl Rng, range: Range<f32>) -> Self {
        Self {
            value: rng.random_range(range),
        }
    }

    #[inline]
    pub fn squared(self) -> f32 {
        self.value * self.value
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            value: self.value.max(other.value),
        }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            value: self.value.min(other.value),
        }
    }

    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        self.value = self.value.clamp(min.value, max.value);
    }
}

impl From<f32> for Radius {
    #[inline]
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<Radius> for f32 {
    #[inline]
    fn from(radius: Radius) -> Self {
        radius.value
    }
}

impl Add for Radius {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Radius {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Radius {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Radius {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul<f32> for Radius {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            value: self.value * scalar,
        }
    }
}

impl MulAssign<f32> for Radius {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.value *= scalar;
    }
}

impl Div<f32> for Radius {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            value: self.value / scalar,
        }
    }
}

impl DivAssign<f32> for Radius {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.value /= scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::Radius;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn radius_ops() {
        let mut radius: Radius = Radius::new(5.0);

        assert_eq!(radius + Radius::new(3.0), Radius::new(8.0));

        assert_eq!(radius * 2.0, Radius::new(10.0));

        radius += Radius::new(1.0);
        assert_eq!(radius, Radius::new(6.0));
    }

    #[test]
    fn radius_math() {
        let radius: Radius = Radius::new(4.0);
        let squared_value: f32 = radius.squared();
        assert_eq!(squared_value, 16.0);
    }

    #[test]
    fn radius_comparison() {
        let radius_a: Radius = Radius::new(2.0);
        let radius_b: Radius = Radius::new(5.0);

        assert!(radius_a < radius_b);

        let max_radius: Radius = radius_a.max(radius_b);
        assert_eq!(max_radius, radius_b);
    }

    #[test]
    fn radius_random() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let random_radius: Radius = Radius::new_random(&mut rng, 10.0f32..20.0f32);

        let value: f32 = random_radius.value;
        assert!(value >= 10.0 && value < 20.0);
    }
}
