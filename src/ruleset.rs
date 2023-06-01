use egui_macroquad::egui::color::rgb_from_hsv;
use macroquad::prelude::Color;
use macroquad::rand;
use crate::helper::*;

pub struct Ruleset {
	pub alpha: f64,
	pub beta: f64,
	pub r: f64,
	pub v: f64,
	pub col1: Color,
	pub col2: Color,
	pub col_scale_min: f32,
	pub col_scale_max: f32,
}

impl Ruleset {
	pub fn from_degrees(alpha: f64, beta: f64, r: f64, v: f64, col1: Color, col2: Color, col_scale_min: f32, col_scale_max: f32) -> Self {
		Ruleset { 
			alpha: alpha/180.0 * std::f64::consts::PI, 
			beta: beta/180.0 * std::f64::consts::PI, 
			r, 
			v, 
			col1,
			col2,
			col_scale_min,
			col_scale_max,
		}
	}

	pub fn random() -> Self {
		Ruleset { 
			alpha: rand::gen_range(-std::f64::consts::PI, std::f64::consts::PI), 
			beta: rand::gen_range(-std::f64::consts::PI, std::f64::consts::PI), 
			r: rand::gen_range(1.0, 40.0), 
			v: rand::gen_range(1.0, 3.0), 
			col1: col_from_array(rgb_from_hsv((rand::gen_range(0.0, 1.0), 1.0, 1.0))),
			col2: col_from_array(rgb_from_hsv((rand::gen_range(0.0, 1.0), 1.0, 1.0))),
			col_scale_min: rand::gen_range(1.0, 10.0),
			col_scale_max: rand::gen_range(30.0, 70.0),
		}
	}

	pub fn randomize(&mut self) {
		self.alpha = rand::gen_range(-std::f64::consts::PI, std::f64::consts::PI);
		self.beta = rand::gen_range(-std::f64::consts::PI, std::f64::consts::PI);
		self.r = rand::gen_range(1.0, 40.0);
		self.v = rand::gen_range(1.0, 3.0);
		self.col1 = col_from_array(rgb_from_hsv((rand::gen_range(0.0, 1.0), 1.0, 1.0)));
		self.col2 = col_from_array(rgb_from_hsv((rand::gen_range(0.0, 1.0), 1.0, 1.0)));
		self.col_scale_min = rand::gen_range(1.0, 10.0);
		self.col_scale_max = rand::gen_range(30.0, 70.0);
	}
}