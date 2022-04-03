use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rapier2d::prelude::*;

use crate::core::physics::*;
use crate::core::transform::Transform2;

fn scale_physics<T>(value: T) -> T {
    value
}

pub fn physics_system_add(
    collider_set: ResMut<ColliderSetRes>,
    rigid_body_set: ResMut<RigidBodySetRes>,
    mut commands: Commands,
    mut rigid_body_entities: ResMut<PhysicsHandleRemovedEntitiesRes>,
    //
    mut query: Query<(Entity, &PhysicsHandle), Added<PhysicsHandle>>,
) {
    for (entity, physics_handle) in query.iter_mut() {
        if rigid_body_set.contains(physics_handle.0) {
            let rigid_body = &rigid_body_set[physics_handle.0];

            for collider_handle in rigid_body.colliders() {
                let rigid_body_collider = &collider_set[*collider_handle];
                let rigid_body_collider_shape = rigid_body_collider.shape();

                match rigid_body_collider_shape.shape_type() {
                    ShapeType::Ball => {
                        let ball = rigid_body_collider_shape.as_ball().unwrap();
                        let circle_shape = shapes::Circle {
                            radius: scale_physics(ball.radius),
                            ..Default::default()
                        };

                        commands.entity(entity).with_children(|child_builder| {
                            child_builder.spawn_bundle(GeometryBuilder::build_as(
                                &circle_shape,
                                DrawMode::Outlined {
                                    fill_mode: FillMode {
                                        color: Color::YELLOW,
                                        options: FillOptions::default(),
                                    },
                                    outline_mode: StrokeMode {
                                        color: Color::BLACK,
                                        options: StrokeOptions::default(),
                                    },
                                },
                                Transform::default(), // TODO: collider transform
                            ));
                        });
                    }
                    ShapeType::Cuboid => {
                        let cuboid = rigid_body_collider_shape.as_cuboid().unwrap();
                        let rectangle_shape = shapes::Rectangle {
                            extents: Vec2::new(
                                scale_physics(cuboid.half_extents[0] * 2.0),
                                scale_physics(cuboid.half_extents[1] * 2.0),
                            ),
                            ..Default::default()
                        };

                        commands.entity(entity).with_children(|child_builder| {
                            child_builder.spawn_bundle(GeometryBuilder::build_as(
                                &rectangle_shape,
                                DrawMode::Outlined {
                                    fill_mode: FillMode {
                                        color: Color::YELLOW,
                                        options: FillOptions::default(),
                                    },
                                    outline_mode: StrokeMode {
                                        color: Color::BLACK,
                                        options: StrokeOptions::default(),
                                    },
                                },
                                Transform::default(), // TODO: collider transform
                            ));
                        });
                    }
                    _ => (),
                };
            }

            rigid_body_entities.insert(entity, physics_handle.0);
        }
    }
}

pub fn physics_system_step(
    gravity: Res<GravityRes>,
    integration_parameters: Res<IntegrationParametersRes>,
    //
    mut joint_set: ResMut<JointSetRes>,
    mut ccd_solver: ResMut<CCDSolverRes>,
    mut broad_phase: ResMut<BroadPhaseRes>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut narrow_phase: ResMut<NarrowPhaseRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut island_manager: ResMut<IslandManagerRes>,
    mut query_pipeline: ResMut<QueryPipelineRes>,
    //
    mut query: Query<(&mut Transform2, &PhysicsHandle)>,
) {
    let hooks = ();
    let events = ();
    let mut physics_pipeline = PhysicsPipeline::new();

    physics_pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
        &mut ccd_solver,
        &hooks,
        &events,
    );

    query_pipeline.update(&island_manager, &rigid_body_set, &collider_set);

    for (mut transform2, physics_handle) in query.iter_mut() {
        if rigid_body_set.contains(physics_handle.0) {
            let rigid_body = &rigid_body_set[physics_handle.0];
            let rigid_body_rotation = rigid_body.rotation();
            let rigid_body_translation = rigid_body.translation();

            transform2.pos.x = scale_physics(rigid_body_translation.x).into();
            transform2.pos.y = scale_physics(rigid_body_translation.y).into();
            transform2.rotation = rigid_body_rotation.angle().into();
        }
    }
}

pub fn physics_system_remove(
    removed_entities: RemovedComponents<PhysicsHandle>,
    //
    mut joint_set: ResMut<JointSetRes>,
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    mut island_manager: ResMut<IslandManagerRes>,
    mut physics_handle_removed_entities: ResMut<PhysicsHandleRemovedEntitiesRes>,
) {
    for removed_entity in removed_entities.iter() {
        if let Some(&rigid_body_handle) = physics_handle_removed_entities.get(&removed_entity) {
            rigid_body_set.remove(
                rigid_body_handle,
                &mut island_manager,
                &mut collider_set,
                &mut joint_set,
            );

            physics_handle_removed_entities.remove(&removed_entity);
        }
    }
}

pub fn physics_system_kinematic(// mut collider_set: ResMut<ColliderSetRes>,
    // mut rigid_body_set: ResMut<RigidBodySetRes>,
    // //
    // mut query: Query<(&mut PhysicsHandle, &PhysicsCollider, &Transform2), With<KinematicBody>>,
) {
    // for (mut handle, collider, transform2) in query.iter_mut() {
    //     if let Some(mut rigid_body) = rigid_body_set.get(handle.0) {}
    // }
}

pub fn physics_startup_system_kinematic_register(
    mut collider_set: ResMut<ColliderSetRes>,
    mut rigid_body_set: ResMut<RigidBodySetRes>,
    //
    mut query: Query<(&mut PhysicsHandle, &PhysicsCollider, &Transform2), With<KinematicBody>>,
) {
    for (mut handle, collider, transform2) in query.iter_mut() {
        let body = RigidBodyBuilder::new_kinematic_velocity_based()
            .rotation(transform2.rotation.into())
            .translation(vector![transform2.pos.x.into(), transform2.pos.y.into()])
            .build();
        let body_handle = rigid_body_set.insert(body);
        let body_collider = ColliderBuilder::cuboid(collider.size.x.into(), collider.size.y.into())
            .restitution(0.0)
            .collision_groups(InteractionGroups::new(collider.layer, collider.layer_mask))
            .build();

        handle.0 = body_handle;
        collider_set.insert_with_parent(body_collider, body_handle, &mut rigid_body_set);
    }
}
