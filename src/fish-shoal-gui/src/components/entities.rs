/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http:
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::Utils;
use eframe::{
    egui::{Painter, Shape, Stroke},
    emath::{Pos2, Vec2},
};
use fish_shoal_simulator::SimulatorOutput;

pub struct Entities;

impl Entities {
    pub fn render_all(painter: Painter, data: SimulatorOutput, origin: Pos2) {
        let count: usize = data.positions.len();

        for i in 0..count {
            Self::render_entity(i, &painter, &data, origin);
        }
    }

    fn render_entity(idx: usize, painter: &Painter, data: &SimulatorOutput, origin: Pos2) {
        let position = data.positions[idx];
        let velocity = data.velocities[idx];
        let speed = data.speeds[idx];

        let screen_pos = origin + Vec2::new(position.x, position.y);
        let color = Utils::speed_to_color(speed);

        if speed > 0.1 {
            let vel_vec = Vec2::new(velocity.x, velocity.y);

            let direction = vel_vec.normalized();

            let size = 6.0;
            let width = 3.0;

            let nose = screen_pos + direction * size;

            let right = Vec2::new(direction.y, -direction.x) * width;
            let tail_base = screen_pos - direction * (size * 0.5);

            let corner_left = tail_base - right;
            let corner_right = tail_base + right;

            painter.add(Shape::convex_polygon(
                vec![nose, corner_right, corner_left],
                color,
                Stroke::NONE,
            ));
        } else {
            painter.circle_filled(screen_pos, 2.0, color);
        }
    }
}
