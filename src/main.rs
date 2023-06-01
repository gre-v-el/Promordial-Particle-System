use macroquad::{prelude::*, rand, time};

mod primordial_particle_system;
use primordial_particle_system::*;

mod particle;
use particle::*;

mod controls;
use controls::*;

mod ruleset;
use ruleset::*;

mod helper;

mod ui;
use ui::*;

#[derive(PartialEq)]
enum ToolAction {
	Draw,
	Erase,
	Move,
	Rerule
}

fn conf() -> Conf {
	Conf { 
		window_title: "Primordial Particle System".into(),
		fullscreen: true, 
		..Default::default()
	}
}

pub struct Tool {
	action: ToolAction,
	radius: f32,
	draw_amount: f64,
	draw_ruleset: usize,
	last_draw: f64,
	dragging: Vec<usize>,
}

#[macroquad::main(conf)]
async fn main() {
	rand::srand(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);

	let mut draw_ui = true;

	let mut playing = true;
	let mut can_drag = true;
	let mut moving;
	let mut controls = Controls::new();
	let mut pps = PrimordialParticleSystem::new();

	let mut tool = Tool {
		action: ToolAction::Draw,
		radius: 10.0,
		draw_amount: 20.0,
		draw_ruleset: 0,
		last_draw: 0.0,
		dragging: Vec::new(),
	};

	loop {
        clear_background(BLACK);

		// controls
		set_camera(controls.camera());
		if can_drag {
			controls.update();
		}
		if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
			let (_, d_zoom) = mouse_wheel();
			tool.radius /= 1.001f32.powf(d_zoom);
		}
		if is_key_pressed(KeyCode::F1) {
			draw_ui = !draw_ui;
		}

		// tool handling
		moving = false;
		if can_drag && is_mouse_button_down(MouseButton::Left) {
			match tool.action {
				ToolAction::Draw => {
					if time::get_time() - tool.last_draw > 1.0f64 / tool.draw_amount {
						pps.add(controls.mouse_world.x as f64, controls.mouse_world.y as f64, tool.radius as f64, tool.draw_ruleset);
						tool.last_draw += 1.0f64 / tool.draw_amount;
						if is_mouse_button_pressed(MouseButton::Left) {
							tool.last_draw = time::get_time();
						}
					}
				},
				ToolAction::Erase => {
					pps.erase(controls.mouse_world.x as f64, controls.mouse_world.y as f64, tool.radius as f64);
				},
				ToolAction::Move => {
					moving = true;
					if is_mouse_button_pressed(MouseButton::Left) {
						tool.dragging = pps.get_all_hovered(controls.mouse_world.x as f64, controls.mouse_world.y as f64, tool.radius as f64);
					}
					pps.drag(&tool.dragging, &controls);
				}
				ToolAction::Rerule => {
					pps.rerule(controls.mouse_world.x as f64, controls.mouse_world.y as f64, tool.radius as f64, tool.draw_ruleset);
				}
			}
		}
		
		// update and draw the particle system
		let left_top = controls.camera().screen_to_world((0.0, 0.0).into());
		let right_bottom = controls.camera().screen_to_world((screen_width(), screen_height()).into());
		let pixel = (controls.camera().screen_to_world((0.0, 0.0).into()) - controls.camera().screen_to_world((0.0, 1.0).into())).y;

		if playing && !moving {
			pps.update();
		}

		let margin = 1.0f32;
		pps.draw(
			Rect::new(
				left_top.x - margin, 
				right_bottom.y - margin, 
				right_bottom.x - left_top.x + 2.0*margin, 
				left_top.y - right_bottom.y + 2.0*margin
			),
			pixel
		);

		// draw cursor
		draw_circle_lines(
			controls.mouse_world.x, 
			controls.mouse_world.y, 
			tool.radius, 
			pixel, 
			Color::from_rgba(100, 100, 100, 255)
		);
		
		// ui
		if draw_ui {
			egui_macroquad::ui(|ctx| {
				// ctx.set_pixels_per_point(1.0);
	
				draw_inspector(&ctx, &mut pps, &mut tool);
				draw_tools(ctx, &mut tool);
				draw_stats(ctx, &mut pps);
				playing = draw_controls(ctx, &mut playing, &mut pps);
				
				can_drag = !(ctx.is_using_pointer() || ctx.is_pointer_over_area());
			});
			egui_macroquad::draw();
		}

        next_frame().await
    }
}