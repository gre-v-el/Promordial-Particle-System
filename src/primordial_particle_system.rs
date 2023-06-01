use macroquad::prelude::Color;
use macroquad::prelude::Rect;
use macroquad::rand;
use macroquad::shapes::draw_line;

use crate::Particle;
use crate::Ruleset;
use crate::controls::Controls;

pub struct PrimordialParticleSystem {
	pub particles: Vec<Particle>,
	pub rulesets: Vec<Ruleset>,
}

impl PrimordialParticleSystem {
	pub fn new() -> Self {

		let mut rulesets: Vec<Ruleset> = Vec::new();
		rulesets.push(
			Ruleset::from_degrees(
				180.0, 
				17.0, 
				15.0, 
				2.0, 
				Color::from_rgba(0, 0, 255, 255),
				Color::from_rgba(255, 100, 0, 255),
				1.0,
				30.0,
			)
		);


		let particles = Vec::new();

		PrimordialParticleSystem {
			particles,
			rulesets,
		}
	}

	pub fn delete_ruleset(&mut self, i: usize) {
		for p in self.particles.iter_mut() {
			if p.ruleset == i {
				p.ruleset = 0;
			}
			else if p.ruleset > i {
				p.ruleset -= 1;
			}
		}

		self.rulesets.remove(i);
	}

	pub fn add(&mut self, x: f64, y: f64, r: f64, ruleset: usize) {
		let dist = r * rand::gen_range(0.0f64, 1.0f64).sqrt();
		let angle = rand::gen_range(0.0f64, 1.0f64) * 2.0 * std::f64::consts::PI;
		self.particles.push(Particle::new(x+dist*angle.cos(), y+dist*angle.sin(), ruleset));
	}

	pub fn erase(&mut self, x: f64, y: f64, r: f64) {
		self.particles.retain(|p| {
			(p.x - x)*(p.x - x) + (p.y - y)*(p.y - y) > r*r
		});
	}

	pub fn drag(&mut self, hovered: &Vec<usize>, controls: &Controls) {
		for i in hovered.iter() {
			self.particles[*i].x += controls.drag.x as f64;
			self.particles[*i].y += controls.drag.y as f64;
			
			self.particles[*i].last_x = self.particles[*i].x;
			self.particles[*i].last_y = self.particles[*i].y;
		}
	}

	pub fn get_all_hovered(&mut self, x: f64, y: f64, r: f64) -> Vec<usize> {
		let mut hovered = Vec::new();

		for (i, p) in self.particles.iter().enumerate() {
			if (p.x - x)*(p.x - x) + (p.y - y)*(p.y - y) < r*r {
				hovered.push(i)
			}
		}

		hovered
	}

	pub fn rerule(&mut self, x: f64, y: f64, r: f64, ruleset: usize) {
		for p in self.particles.iter_mut() {
			if (p.x - x)*(p.x - x) + (p.y - y)*(p.y - y) < r*r {
				p.ruleset = ruleset;
			}
		}
	}

	pub fn update(&mut self) {
		let mut slice = self.particles.as_mut_slice();

		while let Some((first, rest)) = slice.split_first_mut() {
			for second in rest.iter_mut() {
				first.register(&second, &self.rulesets);
				second.register(&first, &self.rulesets);
			}
			first.update(&self.rulesets);
			slice = rest;
		}
	}

	pub fn draw(&self, area: Rect, pixel: f32) {
		let target_lines = 40.0;
		let spacing = (area.w/target_lines).log2().floor().exp2();
		let odd_opacity = ((area.w/target_lines) - spacing)/spacing;
		let odd_opacity = (odd_opacity*3.0).min(1.0);
		let odd_opacity = 1.0 - odd_opacity;
		let opacity = 0.3;

		let start_x = (area.left() / spacing / 2.0).floor()*spacing*2.0;
		let start_y = (area.top() /  spacing / 2.0).floor()*spacing*2.0;
		let steps_x = (area.w / spacing).ceil() as usize;
		let steps_y = (area.h / spacing).ceil() as usize + 1;

		let width = pixel;

		for i in 0..=steps_x {
			let x = i as f32;
			let col = if i%2 == 0 {Color::new(1.0, 1.0, 1.0, opacity)} else {Color::new(1.0, 1.0, 1.0, odd_opacity*opacity)};
			draw_line(start_x + x*spacing, area.top(), start_x + x*spacing, area.bottom(), width, col);
		}
		for i in 0..=steps_y {
			let y = i as f32;
			let col = if i%2 == 0 {Color::new(1.0, 1.0, 1.0, opacity)} else {Color::new(1.0, 1.0, 1.0, odd_opacity*opacity)};
			draw_line(area.left(), start_y + y*spacing, area.right(), start_y + y*spacing, width, col);
		}

		for p in self.particles.iter() {
			if area.contains((p.get_draw_x(), p.get_draw_y()).into()) {
				p.draw(&self.rulesets);
			}
		}
	}

}