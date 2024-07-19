mod despawn;
mod gameplay;
mod level_select;
mod main_menu;

pub use despawn::despawn;

use bevy::prelude::*;
use bevy_obj::ObjPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MainState {
	MainMenu,
	LevelSelect,
	#[default]
	Gameplay,
}

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins,
			ObjPlugin,
			main_menu::MainMenuPlugin,
			gameplay::GameplayPlugin,
		))
		.init_state::<MainState>()
		.run();
}
