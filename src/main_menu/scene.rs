use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuScene;

pub fn spawn(mut commands: Commands) {
	commands.spawn((Camera2dBundle::default(), MainMenuScene));

	commands
		.spawn((
			NodeBundle {
				style: Style {
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				..default()
			},
			MainMenuScene,
		))
		.with_children(|parent| {
			parent.spawn(TextBundle::from_section(
				"Main Menu",
				TextStyle {
					font_size: 100.0,
					color: Color::srgba(8.0, 1.0, 1.0, 1.0),
					..default()
				},
			));
		});
}
