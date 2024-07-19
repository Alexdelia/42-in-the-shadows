use bevy::prelude::*;

#[derive(Component)]
pub struct GameplayScene;

pub fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	// camera
	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		GameplayScene,
	));
	// circular base
	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Circle::new(4.0)),
			material: materials.add(Color::WHITE),
			transform: Transform::from_rotation(Quat::from_rotation_x(
				-std::f32::consts::FRAC_PI_2,
			)),
			..default()
		},
		GameplayScene,
	));
	// obj
	commands.spawn((
		PbrBundle {
			mesh: asset_server.load("42.obj"),
			material: materials.add(Color::srgb_u8(124, 144, 255)),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..default()
		},
		GameplayScene,
	));
	// light
	commands.spawn((
		PointLightBundle {
			point_light: PointLight {
				shadows_enabled: true,
				..default()
			},
			transform: Transform::from_xyz(4.0, 8.0, 4.0),
			..default()
		},
		GameplayScene,
	));
}
