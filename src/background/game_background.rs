use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::states::states::{despawn_screen, GameState};
pub struct GameBackgroundPlugin;

#[derive(Component)]
struct OnGameScreen;

impl Plugin for GameBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), background_setup)
            .add_systems(Update, background_animation)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn spawn_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let star = Star::new();

    // from respawn position.x = +650 not random

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(star.size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(star.color.r(), star.color.g(), star.color.b()))),
            transform: Transform::from_translation(star.position),
            ..default()
        },Star{
            size: star.size,
            color: star.color,
            position: star.position,
            speed: star.speed,
        }
    ));
}
fn background_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    for _ in 0..100 {
        spawn_star(&mut commands, &mut meshes, &mut materials)
    }
}

fn background_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stars : Query<(&mut Transform, &mut Star, Entity), With<Star>>
){

    for (mut transform, star, entity) in &mut stars {
        transform.translation.x += -1. * (star.speed * star.size)* time.delta_seconds();

        if transform.translation.x <= -650. {
            commands.entity(entity).despawn();
            spawn_star(&mut commands, &mut meshes, &mut materials);
        }
    }
}

#[derive(Component)]
pub struct Star {
    size: f32,
    color: Color,
    position: Vec3,
    speed: f32
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
    fn new() -> Star {
        Star{
            size: rand_float(0.5, 2.5),
            color: STAR_COLORS[rand_u(0,6)],
            position: Vec3::new(rand_float(-640., 640.), rand_float(-340., 340.), rand_float(0., 99.)),
            speed: 20.
        }

    }
}
fn rand_float(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
fn rand_u(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}