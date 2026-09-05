pub mod render;

use macroquad::prelude::*;

pub struct Texture {
	pub grass_top: Texture2D,
	pub grass_side: Texture2D,
	pub dirt: Texture2D,
	pub cobblestone: Texture2D,
}
impl Texture {
	pub async fn new() -> Self {
		let textures: [Texture2D; 4] = [
			load_texture("assets/textures/blocks/grass_top.png")
				.await
				.unwrap(),
			load_texture("assets/textures/blocks/grass_side.png")
				.await
				.unwrap(),
			load_texture("assets/textures/blocks/dirt.png")
				.await
				.unwrap(),
			load_texture("assets/textures/blocks/cobblestone.png")
				.await
				.unwrap(),
		];

		for texture in textures.iter() {
			texture.set_filter(FilterMode::Nearest);
		}

		Texture {
			grass_top: textures[0].clone(),
			grass_side: textures[1].clone(),
			dirt: textures[2].clone(),
			cobblestone: textures[3].clone(),
		}
	}
}
