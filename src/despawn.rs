use bevy::prelude::*;

pub fn despawn<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}
