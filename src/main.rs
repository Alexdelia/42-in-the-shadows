mod gameplay;
mod level_select;
mod main_menu;

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
		.init_state::<MainState>()
		.add_plugins((DefaultPlugins, ObjPlugin))
		.add_systems(OnEnter(MainState::MainMenu), main_menu::setup)
		.add_systems(OnEnter(MainState::Gameplay), gameplay::setup)
		.run();
}
