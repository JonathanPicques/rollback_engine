use bevy::math::Quat;
use bevy::prelude::{Changed, Query, Transform};

use crate::core::transform::Transform2;

pub fn sync_transform_system(mut query: Query<(&mut Transform, &Transform2), Changed<Transform2>>) {
    for (mut transform, transform2) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(transform2.rotation.into());
        transform.translation.x = transform2.pos.x.into();
        transform.translation.y = transform2.pos.y.into();
    }
}
