use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{Color, ColorMaterial, Commands, Component, default, Mesh, ResMut, shape, Transform};
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::plugin::game::OnGameScreen;
use crate::screen::CURRENT_MODE;

#[derive(Component)]
pub struct Star {
    pub(crate) size: f32,
    pub(crate) color: Color,
    pub(crate) position: Vec3,
    pub(crate) speed: f32
}

static STAR_COLORS: &'static [Color] = &[
    //blue
    Color::rgb(1.75, 2.01, 2.55),
    //blue pale
    Color::rgb(1.99, 2.16, 2.55),
    //rose pale
    Color::rgb(2.55, 2.44, 2.43),
    //orange pale
    Color::rgb(2.55, 2.29, 2.07),
    //orange 1
    Color::rgb(2.55, 2.17, 1.78),
    //orange 2
    Color::rgb(2.55, 1.99, 1.42),
    //orange 3
    Color::rgb(2.55, 1.66, 0.81)
];

impl Star {
    pub fn new() -> Star {
        let screen = unsafe {CURRENT_MODE.get_resolution()};
        Star{
            size: Star::rand_float(0.5, 2.5),
            color: STAR_COLORS[Star::rand_u(0,6)],
            position: Vec3::new(
                Star::rand_float(-screen.width, screen.width),
                Star::rand_float(-screen.height, screen.height),
                Star::rand_float(0., 99.)
            ),
            speed: 20.
        }

    }

    pub fn rand_float(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..max)
    }
    pub fn rand_u(min: usize, max: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..max)
    }
}

pub fn spawn_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    respawn: bool
) {
    let mut star = Star::new();
    let screen = unsafe {CURRENT_MODE.get_resolution()};

    if respawn {
        star.position = Vec3::new(screen.width, Star::rand_float(-screen.height, screen.height), Star::rand_float(0., 99.));
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(star.size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(star.color.r(), star.color.g(), star.color.b()))),
            transform: Transform::from_translation(star.position),
            ..default()
        }, star
        , OnGameScreen
    ));
}
