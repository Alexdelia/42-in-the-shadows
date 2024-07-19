mod despawn;
mod gameplay;
mod level_select;
mod main_menu;

pub use despawn::despawn;

use bevy::prelude::*;
use bevy_obj::ObjPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MainState {
	#[default]
	MainMenu,
	LevelSelect,
	Gameplay,
}

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, ObjPlugin, main_menu::MainMenuPlugin))
		.init_state::<MainState>()
		.add_systems(OnEnter(MainState::Gameplay), gameplay::setup)
		.run();
}
