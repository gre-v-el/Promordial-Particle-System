use macroquad::prelude::WHITE;
use macroquad::{shapes::draw_circle, prelude::Color, rand};

use crate::ruleset::Ruleset;
use crate::helper::*;

pub struct Particle {
	pub x: f64,
	pub y: f64,
	pub angle: f64,
	pub ruleset: usize,

	others: u32,
	right: u32,
	left: u32,

	pub last_x: f64,
	pub last_y: f64,

	last_others: u32,
}
// make history of positions of length based on the averaging length

impl Particle {

	pub fn new(x: f64, y: f64, ruleset: usize) -> Self {
		Particle { 
			x, 
			y, 
			angle: rand::gen_range(0.0, std::f64::consts::PI * 2.0), 
			ruleset,
			others: 0, right: 0, left: 0,
			last_x: x, last_y: y,
			last_others: 0,
		}
	}

	pub fn register(&mut self, other: &Particle, rulesets: &Vec<Ruleset>) {
		
		let rs = &rulesets[self.ruleset];

		if (other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y) < rs.r * rs.r {
			self.others += 1;

			let mut angle = (other.y - self.y).atan2(other.x - self.x);
			if angle < 0.0 {
				angle += std::f64::consts::PI * 2.0;
			}

			let mut angle_diff = angle - self.angle;
			if angle_diff < 0.0 {
				angle_diff += std::f64::consts::PI * 2.0;
			}


			if angle_diff > 0.0 && angle_diff <= std::f64::consts::PI { // left
				self.left += 1;
			}
			else { // right
				self.right += 1;
			}
		}

	}

	pub fn update(&mut self, rulesets: &Vec<Ruleset>) {
		
		let rs = &rulesets[self.ruleset];

		self.last_x = self.x;
		self.last_y = self.y;

		self.angle += rs.alpha;

		if self.left > self.right {
			self.angle += rs.beta * self.others as f64;
		}
		else if self.right > self.left {
			self.angle -= rs.beta * self.others as f64;
		}

		if self.angle > std::f64::consts::PI * 2.0 {
			self.angle -= std::f64::consts::PI * 2.0;
		}
		else if self.angle < 0.0 {
			self.angle += std::f64::consts::PI * 2.0
		}

		self.x += self.angle.cos() * rs.v;
		self.y += self.angle.sin() * rs.v;

		
		self.last_others = self.others;
		self.others = 0;
		self.right = 0;
		self.left = 0;
	}

	fn get_color(&self, others: u32, rs: &Ruleset) -> Color {
		// let others = 1.0 - scale / (others as f64 + scale);
		// let others = others.powf(3.0);
		// let others = 0.5 + 0.5 * (others as f32 / min).sin();
		// let others = (255.0 * others) as u8;

		let others = ((others as f32 - rs.col_scale_max) / (rs.col_scale_max - rs.col_scale_min) + 1.0).clamp(0.0, 1.0);

		// Color::from_rgba(others, 255 - others, 255, 255)
		// let rgb = rgb_from_hsv((others, 1.0, 1.0));
		// Color::new(rgb[1], rgb[2], rgb[0], 1.0)

		Color::new(
			lerp(rs.col1.r, rs.col2.r, others).clamp(0.0, 1.0),
			lerp(rs.col1.g, rs.col2.g, others).clamp(0.0, 1.0),
			lerp(rs.col1.b, rs.col2.b, others).clamp(0.0, 1.0),
			lerp(rs.col1.a, rs.col2.a, others).clamp(0.0, 1.0),
		)
	}

	pub fn get_draw_x(&self) -> f32 {
		0.5*(self.last_x + self.x) as f32
	}
	pub fn get_draw_y(&self) -> f32 {
		0.5*(self.last_y + self.y) as f32
	}

	pub fn draw(&self, rulesets: &Vec<Ruleset>) {
		let rs = &rulesets[self.ruleset];
		draw_circle(self.get_draw_x(), self.get_draw_y(), 1.0, self.get_color(self.last_others, rs));
	}
}