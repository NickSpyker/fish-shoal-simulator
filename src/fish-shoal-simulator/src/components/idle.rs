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

use crate::Config;
use shipyard::Unique;

#[derive(Unique)]
pub struct Idle {
    pub chance_to_change_direction: f64,
    pub chance_to_change_speed: f64,
    pub chance_to_change_stress: f64,
}

impl Default for Idle {
    fn default() -> Self {
        Self {
            chance_to_change_direction: 0.1,
            chance_to_change_speed: 0.05,
            chance_to_change_stress: 0.001,
        }
    }
}

impl Idle {
    pub fn update(&mut self, config: &Config) {
        self.chance_to_change_direction = config.chance_to_change_direction;
        self.chance_to_change_speed = config.chance_to_change_speed;
        self.chance_to_change_stress = config.chance_to_change_stress;
    }
}
