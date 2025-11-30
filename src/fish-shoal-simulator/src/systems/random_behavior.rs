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

use crate::{Speed, Stress, TargetSpeed, TargetVelocity, Velocity};
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use shipyard::{IntoIter, ViewMut};

const CHANCE_TO_CHANGE_DIRECTION: f64 = 0.1;
const CHANCE_TO_CHANGE_SPEED: f64 = 0.05;
const CHANCE_TO_CHANGE_STRESS: f64 = 0.001;

pub struct RandomBehavior;

impl RandomBehavior {
    pub fn system(
        mut target_velocities: ViewMut<TargetVelocity>,
        mut target_speeds: ViewMut<TargetSpeed>,
        mut stress: ViewMut<Stress>,
    ) {
        (&mut target_velocities, &mut target_speeds, &mut stress)
            .par_iter()
            .for_each(|(target_vel, target_speed, stress)| {
                let mut rng: ThreadRng = rand::rng();

                if rng.random_bool(CHANCE_TO_CHANGE_DIRECTION) {
                    let random_direction = Velocity::new();
                    target_vel.0.lerp(&random_direction, rng.random_range(0.0..1.0));
                }

                if rng.random_bool(CHANCE_TO_CHANGE_SPEED) {
                    let random_speed = Speed::new_random(10.0, 100.0);
                    target_speed.0.lerp(&random_speed, rng.random_range(0.0..1.0));
                }

                if rng.random_bool(CHANCE_TO_CHANGE_STRESS) {
                    let random_stress = Stress::new_random();
                    stress.lerp(&random_stress, rng.random_range(0.0..1.0));
                }
            })
    }
}
