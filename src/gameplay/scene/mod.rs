mod position;

use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct GameplayScene;

pub fn spawn(
	mut c: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	camera(&mut c);
	ambient_light(&mut c);
	spotlight(&mut c);

	wall(&mut c, &mut meshes, &mut materials);

	#[cfg(debug_assertions)]
	{
		let point = meshes.add(Sphere::new(0.3));

		for (pos, color) in &[
			(Vec3::new(0.0, 0.0, 0.0), Color::srgb(1.0, 1.0, 1.0)),
			(position::TOP_FRONT_CORNER, Color::srgb(1.0, 0.0, 0.0)),
			(position::BOT_FRONT_CORNER, Color::srgb(0.0, 0.5, 1.0)),
			(position::TOP_RIGHT_CORNER, Color::srgb(1.0, 1.0, 0.0)),
			(position::BOT_RIGHT_CORNER, Color::srgb(0.5, 0.0, 1.0)),
			(position::TOP_LEFT_CORNER, Color::srgb(0.0, 1.0, 0.0)),
			(position::BOT_LEFT_CORNER, Color::srgb(1.0, 0.0, 1.0)),
			(position::TOP_BACK_CORNER, Color::srgb(0.3, 0.3, 0.3)),
			(position::BOT_BACK_CORNER, Color::srgb(0.0, 0.0, 0.0)),
		] {
			c.spawn((
				PbrBundle {
					mesh: point.clone(),
					material: materials.add(StandardMaterial {
						base_color: *color,
						alpha_mode: AlphaMode::Mask(0.3),
						..default()
					}),
					transform: Transform::from_xyz(pos.x, pos.y, pos.z),
					..default()
				},
				GameplayScene,
			));
		}
	}

	// obj
	/*
	c.spawn((
		PbrBundle {
			mesh: asset_server.load("42.obj"),
			material: materials.add(Color::srgb_u8(124, 144, 255)),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..default()
		},
		GameplayScene,
	));
	*/
}

fn camera(c: &mut Commands) {
	c.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(2.0, 1.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		GameplayScene,
	));
}

fn ambient_light(c: &mut Commands) {
	c.insert_resource(AmbientLight {
		color: Color::WHITE,
		brightness: 20.0,
	});
}

fn spotlight(c: &mut Commands) {
	c.spawn((
		SpotLightBundle {
			transform: Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
			spot_light: SpotLight {
				intensity: 40_000.0, // lumens
				color: Color::WHITE,
				shadows_enabled: true,
				inner_angle: PI / 4.0 * 0.85,
				outer_angle: PI / 4.0,
				..default()
			},
			..default()
		},
		GameplayScene,
	));
}

fn wall(
	c: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) {
	// wall with 4 vertices:
	// - TOP_FRONT_CORNER
	// - BOT_FRONT_CORNER
	// - TOP_RIGHT_CORNER
	// - BOT_RIGHT_CORNER
	c.spawn((
		PbrBundle {
			mesh: meshes.add(Rectangle::new(4.0, 4.0)),
			material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
			// transform: Transform::from_matrix(Mat4::from_cols_array(&[
			// 	position::TOP_FRONT_CORNER.x,
			// 	position::TOP_FRONT_CORNER.y,
			// 	position::TOP_FRONT_CORNER.z,
			// 	0.0, // x
			// 	position::BOT_FRONT_CORNER.x,
			// 	position::BOT_FRONT_CORNER.y,
			// 	position::BOT_FRONT_CORNER.z,
			// 	0.0, // y
			// 	position::TOP_RIGHT_CORNER.x,
			// 	position::TOP_RIGHT_CORNER.y,
			// 	position::TOP_RIGHT_CORNER.z,
			// 	0.0, // z
			// 	0.0,
			// 	0.0,
			// 	0.0,
			// 	1.0,
			// ])),
			transform: Transform::from_matrix(
			..default()
		},
		GameplayScene,
	));

	// c.spawn((
	// 	PbrBundle {
	// 		mesh: meshes.add(Rectangle::new(1.0, 1.0)),
	// 		material: materials.add(Color::srgb_u8(100, 200, 200)),
	// 		transform: Transform::from_xyz(5.0, 0.5, 0.0),
	// 		..default()
	// 	},
	// 	GameplayScene,
	// ));
}
