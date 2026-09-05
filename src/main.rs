mod player;
mod texture;
mod world;
use player::*;
use texture::{render::render, *};
use world::*;

use enigo::{Coordinate, Enigo, Mouse, Settings};
use macroquad::prelude::*;
use std::time::{Duration, Instant};

const TPS: f32 = 20.0;

#[allow(unused_macros)]
macro_rules! elapsed {
	($name:expr, $block:block) => {
		let start = std::time::Instant::now();
		$block;
		let duration = start.elapsed();
		println!("{} took {:?}", $name, duration);
	};
}

fn window() -> Conf {
	Conf {
		window_title: "Minecraft: Rust Edition".to_string(),
		fullscreen: true,
		window_resizable: false,
		sample_count: 1,
		..Default::default()
	}
}

#[macroquad::main(window)]
async fn main() {
	println!("Game start");

	let tick_rate: Duration = Duration::from_secs_f32(1.0 / TPS);
	let mut last_tick: Instant = Instant::now();
	let mut accumlator: Duration = Duration::ZERO;
	println!("Tick rate init");

	let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
	println!("Enigo init");

	let mut player: Player = Player::new();
	let mut world: World = World::new();
	let mut look_angle: Vec2 = Vec2::ZERO;
	let textures: Texture = Texture::new().await;
	let mut running: bool = false;
	println!("Game var init");

	macroquad::texture::build_textures_atlas();
	println!("Atlas init");

	const CHUNK_X_MIN: i32 = -2;
	const CHUNK_X_MAX: i32 = 2;
	const CHUNK_Y_MIN: i32 = 0;
	const CHUNK_Y_MAX: i32 = 4;
	const CHUNK_Z_MIN: i32 = -2;
	const CHUNK_Z_MAX: i32 = 2;

	for (z, y, x) in (CHUNK_Z_MIN..CHUNK_Z_MAX).flat_map(|z: i32| {
		(CHUNK_Y_MIN..CHUNK_Y_MAX).flat_map(move |y: i32| {
			(CHUNK_X_MIN..CHUNK_X_MAX).map(move |x: i32| -> (i32, i32, i32) { (z, y, x) })
		})
	}) {
		world.new_chunk(
			x,
			y,
			z,
			&textures,
			if y == 3 {
				ChunkType::OnGround
			} else {
				ChunkType::BelowGround
			},
		);
	}
	println!("Chunk init");

	set_cursor_grab(true);
	show_mouse(false);
	println!("mouse init");

	let (ww, wh) = enigo.main_display().unwrap_or((1920, 1080));
	let window_width: f32 = ww as f32;
	let window_height: f32 = wh as f32;
	let centre_x: f32 = window_width / 2.0;
	let centre_y: f32 = window_height / 2.0;
	println!("Get window width and height");

	println!("Game start");
	loop {
		if is_key_pressed(KeyCode::Escape) {
			println!("Detect esc, game stopping");
			break;
		}

		let elapsed: Duration = last_tick.elapsed();
		last_tick = Instant::now();
		accumlator += elapsed;

		if is_any_key_down() {
			running = true;
		}

		if running {
			while accumlator >= tick_rate {
				game_tick(&mut player, look_angle);
				accumlator -= tick_rate;
			}
		} else {
			accumlator = Duration::ZERO;
		}

		camera_move(centre_x, centre_y, &mut look_angle, &mut enigo);
		render(&player, &mut world, look_angle);

		next_frame().await;
	}

	println!("Game ends");
}

fn game_tick(player: &mut Player, look_angle: Vec2) {
	player.move_player(look_angle);
}

fn camera_move(centre_x: f32, centre_y: f32, look_angle: &mut Vec2, enigo: &mut Enigo) {
	let mouse_loc: (i32, i32) = enigo.location().unwrap();
	let mouse_pos: Vec2 = vec2(mouse_loc.0 as f32, mouse_loc.1 as f32);
	let mut mouse_rel_pos: Vec2 = mouse_pos - vec2(centre_x, centre_y);
	mouse_rel_pos.y *= -1.0;
	mouse_rel_pos *= vec2(0.25, 0.25);
	*look_angle += mouse_rel_pos;
	look_angle.x = (look_angle.x + 180.0) % 360.0 - 180.0;
	look_angle.y = look_angle.y.clamp(-90.0, 90.0);

	enigo
		.move_mouse(centre_x as i32, centre_y as i32, Coordinate::Abs)
		.ok();
}
