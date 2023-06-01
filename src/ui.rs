use std::f64::INFINITY;

use egui_macroquad::egui::{Context, Layout, Rgba, RichText, Ui, WidgetText};
use macroquad::prelude::*;
use egui_macroquad::egui::{self};

use crate::{helper::*, primordial_particle_system::PrimordialParticleSystem, Tool, ruleset::Ruleset, ToolAction};

fn text_slider<Num: egui::emath::Numeric>
	(ui: &mut Ui, text: impl Into<WidgetText>, value: &mut Num, range: std::ops::RangeInclusive<Num>) 
{
	ui.vertical(|ui| {
		ui.label(text);
		ui.add(egui::Slider::new(value, range));
	});
}
fn text_drag_value<Num: egui::emath::Numeric>
	(ui: &mut Ui, text: impl Into<WidgetText>, value: &mut Num, range: std::ops::RangeInclusive<Num>, speed: f64) 
{
	ui.vertical(|ui| {
		ui.label(text);
		ui.add(egui::DragValue::new(value).clamp_range(range).speed(speed));
	});
}

pub fn draw_inspector(ctx: &Context, pps: &mut PrimordialParticleSystem, tool: &mut Tool) {
	egui::Window::new("Rule Inspector")
		.scroll2([false, true])
		.fixed_pos((10.0, 10.0))
		.fixed_size((260.0, screen_height()-165.0))
		.resizable(false)
		.collapsible(false)
		.show(ctx, |ui| {
			
			let mut delete: Option<usize> = Option::None;

			let enabled = pps.rulesets.len() != 1;
			for (i, ruleset) in pps.rulesets.iter_mut().enumerate() {

				ui.horizontal(|ui| {
					ui.radio_value(&mut tool.draw_ruleset, i, egui::RichText::new(format!("ruleset {}", i)).strong().size(15.0));
				});

				egui::Grid::new(format!("ruleset {} grid", i)).spacing((20.0, 7.0)).show(ui, |ui| {
					text_slider(ui, "alpha:", &mut ruleset.alpha, -std::f64::consts::PI..=std::f64::consts::PI);
					text_drag_value(ui, "radius:", &mut ruleset.r, 0.0f64..=INFINITY, 0.1);

					ui.end_row();

					text_slider(ui, "beta:", &mut ruleset.beta, -std::f64::consts::PI..=std::f64::consts::PI);
					text_drag_value(ui, "speed:", &mut ruleset.v, 0.0..=INFINITY, 0.1);

					ui.end_row();
					
					text_slider(ui, "Drawing scale min:", &mut ruleset.col_scale_min, 0.0..=100.0);

					let mut col = [ruleset.col1.r, ruleset.col1.g, ruleset.col1.b];
					egui::widgets::color_picker::color_edit_button_rgb(ui, &mut col);
					ruleset.col1 = col_from_array(col);

					ui.end_row();

					text_slider(ui, "Drawing scale max:", &mut ruleset.col_scale_max, 0.0..=100.0);

					let mut col = [ruleset.col2.r, ruleset.col2.g, ruleset.col2.b];
					egui::widgets::color_picker::color_edit_button_rgb(ui, &mut col);
					ruleset.col2 = col_from_array(col);
				});
				
				ui.horizontal(|ui| {
					if ui.add_enabled(enabled, 
						egui::Button::new(RichText::new("delete").color(Rgba::from_rgb(1.0, 0.0, 0.0))))
						.clicked() 
					{
						delete = Option::Some(i);
					}
					if ui.button("randomize").clicked() {
						ruleset.randomize();
					}
				});
				
				ui.separator();
			}

			if let Option::Some(i) = delete {
				pps.delete_ruleset(i);
				if tool.draw_ruleset == i {
					tool.draw_ruleset = 0;
				}
				else if tool.draw_ruleset > i {
					tool.draw_ruleset -= 1;
				}
			}

			ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
				if ui.button("add new").clicked() {
					pps.rulesets.push(Ruleset::random());
				}
			});
		}
	);
}

pub fn draw_tools(ctx: &Context, tool: &mut Tool) {
	egui::Window::new("Tools")
		.fixed_pos((screen_width() - 143.0, 10.0))
		.resizable(false)
		.collapsible(false)
		.show(ctx, |ui| {
			ui.set_width(120.0);

			ui.radio_value(&mut tool.action, ToolAction::Draw, 	"Draw");
			ui.radio_value(&mut tool.action, ToolAction::Erase, "Erase");
			ui.radio_value(&mut tool.action, ToolAction::Move, 	"Move");
			ui.radio_value(&mut tool.action, ToolAction::Rerule,"Rerule");


			ui.separator();

			ui.label("radius:");
			ui.add(egui::DragValue::new(&mut tool.radius).speed(0.5).clamp_range(0.0..=INFINITY));

			ui.label("draw amount:");
			ui.add(egui::DragValue::new(&mut tool.draw_amount).speed(0.1).clamp_range(-1.0..=INFINITY));
		}
	);
}

pub fn draw_stats(ctx: &Context, pps: &mut PrimordialParticleSystem) {
	egui::Window::new("Stats")
		.fixed_pos((10.0, screen_height() - 110.0))
		.fixed_size((260.0, 100.0))
		.resizable(false)
		.collapsible(false)
		.show(ctx, |ui| {

			ui.label(format!("frame time: {:.2}ms", get_frame_time()*1000.0));
			ui.label(format!("fps: {}", get_fps()));

			ui.separator();

			ui.label(format!("particles: {}", pps.particles.len()));

			ui.set_width(230.0);
		}
	);
}

pub fn draw_controls(ctx: &Context, playing: &bool, pps: &mut PrimordialParticleSystem) -> bool {
	egui::Window::new("Controls")
		.fixed_pos((screen_width() - 143.0, screen_height() - 70.0))
		.resizable(false)
		.collapsible(false)
		.show(ctx, |ui| {
			ui.set_width(120.0);
			let mut ret = *playing;

			ui.horizontal(|ui| {
				if ui.button(if *playing {"pause"} else {"play"}).clicked() {
					ret = !playing;
				}
	
				if ui.button("clear").clicked() {
					pps.particles.clear();
				}
			});
			

			return ret;
		}
	).unwrap().inner.unwrap()
}