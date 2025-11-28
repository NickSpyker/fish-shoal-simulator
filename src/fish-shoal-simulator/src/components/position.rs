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

#[derive(Component, Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new_zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn new_random(x_low: f32, x_high: f32, y_low: f32, y_high: f32) -> Self {
        let mut rng: ThreadRng = rand::rng();

        let x: f32 = rng.random_range(x_low..=x_high);
        let y: f32 = rng.random_range(y_low..=y_high);

        Self::new(x, y)
    }
}
