mod scene;

use bevy::prelude::*;

use crate::{despawn, MainState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(MainState::MainMenu), scene::spawn)
			.add_systems(OnExit(MainState::MainMenu), despawn::<scene::MainMenuScene>);
	}
}
