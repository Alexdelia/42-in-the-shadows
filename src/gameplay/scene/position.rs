use bevy::prelude::Vec3;

pub const DIMENSIONS: f32 = 1.0;

pub const WALL_DIMENSIONS: Vec3 = Vec3::new(12.0 * DIMENSIONS, 9.0 * DIMENSIONS, 12.0 * DIMENSIONS);

pub const TOP_FRONT_CORNER: Vec3 =
	Vec3::new(0.0, WALL_DIMENSIONS.y / 2.0, -(WALL_DIMENSIONS.z / 2.0));
pub const BOT_FRONT_CORNER: Vec3 =
	Vec3::new(0.0, -(WALL_DIMENSIONS.y / 2.0), -(WALL_DIMENSIONS.z / 2.0));

pub const TOP_RIGHT_CORNER: Vec3 = Vec3::new(WALL_DIMENSIONS.x / 2.0, WALL_DIMENSIONS.y / 2.0, 0.0);
pub const BOT_RIGHT_CORNER: Vec3 =
	Vec3::new(WALL_DIMENSIONS.x / 2.0, -(WALL_DIMENSIONS.y / 2.0), 0.0);

pub const TOP_LEFT_CORNER: Vec3 =
	Vec3::new(-(WALL_DIMENSIONS.x / 2.0), WALL_DIMENSIONS.y / 2.0, 0.0);
pub const BOT_LEFT_CORNER: Vec3 =
	Vec3::new(-(WALL_DIMENSIONS.x / 2.0), -(WALL_DIMENSIONS.y / 2.0), 0.0);

pub const TOP_BACK_CORNER: Vec3 = Vec3::new(0.0, WALL_DIMENSIONS.y / 2.0, WALL_DIMENSIONS.z / 2.0);
pub const BOT_BACK_CORNER: Vec3 =
	Vec3::new(0.0, -(WALL_DIMENSIONS.y / 2.0), WALL_DIMENSIONS.z / 2.0);

pub const OBJ: Vec3 = Vec3::new(0.0, 0.0, 0.0);
