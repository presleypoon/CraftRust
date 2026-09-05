mod change_pos;

use macroquad::prelude::*;

/// `pos` is position <br>
/// `vel` is velocity
pub struct Player {
	pub pos: Vec3,
	pub vel: Vec3,
}
impl Player {
	pub fn new() -> Self {
		Player {
			pos: vec3(0.0, 72.0, 0.0),
			vel: vec3(0.0, 0.0, 0.0),
		}
	}
}
