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
use rand::prelude::ThreadRng;
use rand::Rng;
use shipyard::Component;

#[derive(Component, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Stress(f32);

impl Default for Stress {
    fn default() -> Self {
        Self(0.1)
    }
}

impl Stress {
    pub fn new(factor: f32) -> Self {
        Self(factor)
    }

    pub fn new_random() -> Self {
        let mut rng: ThreadRng = rand::rng();

        let random_factor: f32 = rng.random_range(0.1..0.5);

        Self::new(random_factor)
    }

    pub fn factor(&self) -> f32 {
        self.0
    }

    pub fn lerp(&mut self, to: &Self, factor: f32) -> Self {
        let new_factor: f32 = self.0 + (to.0 - self.0) * factor;
        self.0 = new_factor;
        self.clone()
    }
}
