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

use rand::{rngs::ThreadRng, Rng};
use shipyard::Component;
use std::ops::Mul;

#[derive(Component, Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Speed(pub f32);

impl Mul<f32> for &Speed {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }

    pub fn new_zero() -> Self {
        Self::new(0.0)
    }

    pub fn new_random(low: f32, high: f32) -> Self {
        let mut rng: ThreadRng = rand::rng();

        let speed: f32 = rng.random_range(low..=high);

        Self::new(speed)
    }
}
