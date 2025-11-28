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

use crate::Error;
use eframe::{
    egui::{Context, ViewportBuilder}, epaint::{Color32, Pos2}, App,
    Frame,
    NativeOptions,
};
use fish_shoal_simulator::Position;

#[derive(Default)]
pub struct FishShoalGui {
    positions: Vec<Position>,
}

impl FishShoalGui {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        eframe::run_native(
            "Fish Shoal Simulator",
            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_min_inner_size([384.0, 216.0])
                    //.with_inner_size([1024.0, 576.0])
                    .with_maximized(true),
                centered: true,
                ..Default::default()
            },
            Box::new(|_| Ok(Box::new(Self::new()))),
        )
        .map_err(|err| Error::EFrame(err))
    }
}

impl App for FishShoalGui {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter_at(rect);

            for position in &self.positions {
                painter.circle_filled(Pos2::new(position.x, position.y), 2.0, Color32::RED);
            }
        });

        ctx.request_repaint();
    }
}
