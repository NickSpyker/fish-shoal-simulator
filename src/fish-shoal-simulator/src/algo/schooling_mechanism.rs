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

use crate::{Scalar, Vec2};
use rand::Rng;
use shipyard::EntityId;
use std::{cmp::Ordering, collections::HashMap, f32::consts::PI};

#[derive(Debug, Default)]
pub struct SchoolingMechanism {
    position: Vec2,
    velocity: Vec2,
    speed: Scalar,
    others_positions: HashMap<EntityId, Vec2>,
    others_velocities: HashMap<EntityId, Vec2>,
    others_speeds: HashMap<EntityId, Scalar>,
    avoidance_radius: f32,
    alignment_radius: f32,
    attraction_radius: f32,
}

impl SchoolingMechanism {
    pub fn setup(
        position: Vec2,
        velocity: Vec2,
        speed: Scalar,
        others_positions: HashMap<EntityId, Vec2>,
        others_velocities: HashMap<EntityId, Vec2>,
        others_speeds: HashMap<EntityId, Scalar>,
        avoidance_radius: f32,
        alignment_radius: f32,
        attraction_radius: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            speed,
            others_positions,
            others_velocities,
            others_speeds,
            avoidance_radius,
            alignment_radius,
            attraction_radius,
        }
    }

    pub fn set_behavior(&self, velocity: &mut Vec2, speed: &mut Scalar) {
        *velocity = self.velocity;
        *speed = self.speed;
    }

    pub fn update(&mut self, rng: &mut impl Rng) {
        const K: f32 = 4.0;
        const A: f32 = 3.3;
        const SD1: f32 = 15.0 * (PI / 180.0);
        const SD2: f32 = 15.0 * (PI / 180.0);
        const AR: f32 = 150.0 * (PI / 180.0);
        const RF: f32 = 0.5;

        let new_speed: f32 = Self::sample_gamma(rng, K, 1.0 / A);
        self.speed = Scalar::new(new_speed) * 10.0;

        let current_heading: f32 = f32::atan2(self.velocity.y, self.velocity.x);

        let mut candidates: Vec<(f32, f32, Vec2, Vec2)> = Vec::new();

        for (id, pos) in &self.others_positions {
            let neighbor_vel: Vec2 = self
                .others_velocities
                .get(id)
                .copied()
                .unwrap_or(Vec2::ZERO);

            let to_neighbor: Vec2 = *pos - self.position;
            let dist: f32 = to_neighbor.length();

            if dist <= 0.0001 {
                continue;
            }

            let angle_to: f32 = f32::atan2(to_neighbor.y, to_neighbor.x);
            let angle_diff: f32 = Self::angle_difference(current_heading, angle_to);

            if dist < self.attraction_radius && angle_diff.abs() < (AR / 2.0) {
                candidates.push((dist, angle_diff.abs(), *pos, neighbor_vel));
            }
        }

        let chosen_neighbor: Option<(f32, f32, Vec2, Vec2)> = if !candidates.is_empty() {
            candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

            Self::select_neighbor_probabilistic(rng, &candidates, RF).copied()
        } else {
            let mut far_candidates: Vec<(f32, f32, Vec2, Vec2)> = Vec::new();

            for (id, pos) in &self.others_positions {
                let neighbor_vel: Vec2 = self
                    .others_velocities
                    .get(id)
                    .copied()
                    .unwrap_or(Vec2::ZERO);

                let to_neighbor: Vec2 = *pos - self.position;
                let dist: f32 = to_neighbor.length();
                if dist <= 0.0001 {
                    continue;
                }

                let angle_to: f32 = f32::atan2(to_neighbor.y, to_neighbor.x);
                let angle_diff: f32 = Self::angle_difference(current_heading, angle_to);

                if angle_diff.abs() < (AR / 2.0) {
                    far_candidates.push((dist, angle_diff.abs(), *pos, neighbor_vel));
                }
            }

            if !far_candidates.is_empty() {
                far_candidates
                    .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

                Self::select_neighbor_probabilistic(rng, &far_candidates, RF).copied()
            } else {
                None
            }
        };

        let new_dir_angle: f32 = if let Some((dist, _, pos, vel)) = chosen_neighbor {
            let to_neighbor: Vec2 = pos - self.position;
            let angle_to_neighbor: f32 = f32::atan2(to_neighbor.y, to_neighbor.x);

            if dist < self.avoidance_radius {
                let a1: f32 = angle_to_neighbor + PI / 2.0;
                let a2: f32 = angle_to_neighbor - PI / 2.0;

                let diff1: f32 = Self::angle_difference(current_heading, a1).abs();
                let diff2: f32 = Self::angle_difference(current_heading, a2).abs();
                let target_mean: f32 = if diff1 < diff2 { a1 } else { a2 };

                Self::sample_normal(rng, target_mean, SD1)
            } else if dist < self.alignment_radius {
                let neighbor_heading: f32 = f32::atan2(vel.y, vel.x);
                Self::sample_normal(rng, neighbor_heading, SD2)
            } else {
                Self::sample_normal(rng, angle_to_neighbor, SD1)
            }
        } else {
            rng.random_range(0.0..2.0 * PI)
        };

        let (sin, cos): (f32, f32) = new_dir_angle.sin_cos();
        self.velocity = Vec2::new(cos, sin) * self.speed.value;
    }

    fn angle_difference(a: f32, b: f32) -> f32 {
        let mut diff: f32 = b - a;
        while diff > PI {
            diff -= 2.0 * PI;
        }
        while diff < -PI {
            diff += 2.0 * PI;
        }
        diff
    }

    fn select_neighbor_probabilistic<'a, T>(
        rng: &mut impl Rng,
        candidates: &'a [T],
        rf: f32,
    ) -> Option<&'a T> {
        let n: usize = candidates.len();
        if n == 0 {
            return None;
        }

        let mut weights: Vec<f32> = Vec::with_capacity(n);
        let mut sum_w: f32 = 0.0;
        let mut w: f32 = 1.0;
        for _ in 0..n {
            weights.push(w);
            sum_w += w;
            w *= rf;
        }

        let r: f32 = rng.random_range(0.0..sum_w);
        let mut accum: f32 = 0.0;
        for (i, &weight) in weights.iter().enumerate() {
            accum += weight;
            if r < accum {
                return Some(&candidates[i]);
            }
        }
        Some(&candidates[n - 1])
    }

    fn sample_gamma(rng: &mut impl Rng, k: f32, theta: f32) -> f32 {
        if k < 1.0 {
            return Self::sample_gamma(rng, k + 1.0, theta)
                * rng.random_range(0.0f32..1.0f32).powf(1.0 / k);
        }

        let d: f32 = k - 1.0 / 3.0;
        let c: f32 = 1.0 / (9.0 * d).sqrt();

        loop {
            let x: f32 = Self::sample_normal_std(rng);
            let v_term: f32 = 1.0 + c * x;
            if v_term <= 0.0 {
                continue;
            }

            let v: f32 = v_term * v_term * v_term;
            let u: f32 = rng.random_range(0.0f32..1.0f32);

            if u < 1.0 - 0.0331 * x * x * x * x {
                return d * v * theta;
            }
            if u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln()) {
                return d * v * theta;
            }
        }
    }

    fn sample_normal(rng: &mut impl Rng, mean: f32, std: f32) -> f32 {
        mean + Self::sample_normal_std(rng) * std
    }

    fn sample_normal_std(rng: &mut impl Rng) -> f32 {
        let u1: f32 = rng.random_range(0.0f32..1.0f32);
        let u2: f32 = rng.random_range(0.0f32..1.0f32);
        let u1: f32 = if u1 < 1e-10 { 1e-10 } else { u1 };
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}
