use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::plugin::game::OnGameScreen;
use crate::screen::CURRENT_MODE;

#[derive(Component, Clone, Copy)]
pub struct Projectil {
    pub(crate) size: f32,
    pub(crate) position: Vec3,
    pub(crate) speed: f32
}

impl Projectil {
    pub fn new(position: Vec3) -> Projectil {
        Projectil {
            size: 5.,
            position,
            speed: 800.
        }
    }
}

pub fn spwan_projectil(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3
) {
    let projectil = Projectil::new(position);

    commands.spawn((
        MaterialMesh2dBundle {
           mesh: meshes.add(shape::Quad::new(Vec2::new(projectil.size*1.5, projectil.size/2.0)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(2.45, 1.85, 0.66))),
            transform: Transform::from_translation(projectil.position),
            ..default()
        },
        projectil,
        OnGameScreen
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(projectil.size/1.5).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(2.45, 1.85, 0.66))),
            transform: Transform::from_translation(Vec3::new(projectil.position.x+5.0, projectil.position.y, projectil.position.z)),
            ..default()
        },
        projectil,
        OnGameScreen
    ));
}

pub fn projectil_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut projectils : Query<(&mut Transform, &mut Projectil, Entity), With<Projectil>>
) {
    let screen = unsafe {CURRENT_MODE.get_resolution()};

    for (mut transform, projectil, entity) in &mut projectils {

        transform.translation.x += 1. + projectil.speed * time.delta_seconds();

        // despawn on right screen
        if transform.translation.x >= screen.width {
            commands.entity(entity).despawn();
        }
    }
}