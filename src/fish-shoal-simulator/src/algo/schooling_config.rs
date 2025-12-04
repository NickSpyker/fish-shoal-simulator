/*
* Copyright 2025 Nicolas Spijkerman
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http:
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

use std::f32::consts::PI;

/// All inputs required to execute one time-step of the schooling algorithm.
#[derive(Debug, Copy, Clone)]
pub struct SchoolingConfig {
    /// Gamma distribution parameter K for speed generation.
    pub gamma_dist_k: f32,
    /// Gamma distribution parameter A for speed generation.
    pub gamma_dist_a: f32,
    /// If a neighbor is closer than this, the fish steers away.
    pub avoidance_radius: f32,
    /// If a neighbor is in range, the fish aligns (parallel).
    pub alignment_radius: f32,
    /// If a neighbor is within range, the fish move closer.
    pub attraction_radius: f32,
    /// Angular range of interactions. The visual field and blind spot limit.
    pub visual_field: f32,
    /// Standard deviation for avoidance and attraction variance.
    pub avoidance_attraction_standard_deviation: f32,
    /// Standard deviation for parallel orientation variance.
    pub alignment_standard_deviation: f32,
    /// Determines how much influence neighbors have based on relative angle.
    pub reference_factor: f32,
}

impl Default for SchoolingConfig {
    fn default() -> Self {
        let standard_deviation: f32 = 15.0 * (PI / 180.0);

        Self {
            gamma_dist_k: 4.0,
            gamma_dist_a: 3.3,
            avoidance_radius: 50.0,
            alignment_radius: 30.0,
            attraction_radius: 15.0,
            visual_field: 150.0 * (PI / 180.0),
            avoidance_attraction_standard_deviation: standard_deviation,
            alignment_standard_deviation: standard_deviation,
            reference_factor: 0.5,
        }
    }
}
