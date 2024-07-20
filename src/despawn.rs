use bevy::prelude::*;

pub fn despawn<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}

pub fn remove_resource<T: Resource>(mut commands: Commands) {
	commands.remove_resource::<T>();
}
