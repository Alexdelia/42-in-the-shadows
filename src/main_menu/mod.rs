mod scene;

use bevy::prelude::*;

use crate::{despawn, MainState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(MainState::MainMenu), scene::spawn)
			.add_systems(OnExit(MainState::MainMenu), despawn::<scene::MainMenuScene>)
			.add_systems(Update, key_handler);
	}
}

fn key_handler(keys: Res<ButtonInput<KeyCode>>, mut next_main_state: ResMut<NextState<MainState>>) {
	for key in keys.get_just_pressed() {
		match key {
			KeyCode::Enter => next_main_state.set(MainState::Gameplay),
			_ => {}
		}
	}
}
