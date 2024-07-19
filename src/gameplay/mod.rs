mod scene;

use bevy::prelude::*;

use crate::{despawn, MainState};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(MainState::Gameplay), scene::spawn)
			.add_systems(OnExit(MainState::Gameplay), despawn::<scene::GameplayScene>)
			.add_systems(Update, key_handler);
	}
}

fn key_handler(keys: Res<ButtonInput<KeyCode>>, mut next_main_state: ResMut<NextState<MainState>>) {
	for key in keys.get_just_pressed() {
		match key {
			KeyCode::Escape => next_main_state.set(MainState::MainMenu),
			_ => {}
		}
	}
}
