use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::states::game::OnGameScreen;

#[derive(Component)]
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
           mesh: meshes.add(shape::Quad::new(Vec2::new(projectil.size*2.0, projectil.size/2.0)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(2.45, 1.85, 0.66))),
            transform: Transform::from_translation(projectil.position),
            ..default()
        },
        projectil,
        OnGameScreen
    ));
}