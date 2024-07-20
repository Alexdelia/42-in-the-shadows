mod despawn;
mod gameplay;
mod level_select;
mod main_menu;

pub use despawn::{despawn, remove_resource};

use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_obj::ObjPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MainState {
	MainMenu,
	LevelSelect,
	#[default]
	Gameplay,
}

fn main() {
	let mut app = App::new();

	app.add_plugins((
		DefaultPlugins,
		ObjPlugin,
		main_menu::MainMenuPlugin,
		gameplay::GameplayPlugin,
	))
	.init_state::<MainState>();

	#[cfg(debug_assertions)]
	app.add_plugins(WorldInspectorPlugin::new());

	app.run();
}
