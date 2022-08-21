use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

mod beams;
mod enemies;
mod mouse;
mod player;

use beams::*;
use enemies::*;
use mouse::*;
use player::*;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<BeamMaterial>::default())
        .init_resource::<MousePos>()
        .insert_resource(ClosestBeam(BeamColor::Green))
        .insert_resource(EnemySpawnerTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(move_light_beam)
        .add_system(update_time)
        .add_system(update_mouse_pos)
        .add_system(update_closest_beam)
        .add_system(spawn_enemies)
        .add_system(move_enemies)
        .add_system(damage_enemies)
        .run();
}

const BEAM_LENGTH: f32 = 1000.0;
const ENEMY_RADIUS: f32 = 50.0;

fn setup(
    mut commands: Commands,
    a: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut beam_mats: ResMut<Assets<BeamMaterial>>,
) {
    a.watch_for_changes().unwrap();

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));

    let beams = [
        (BeamColor::Red, Vec2::new(-500.0, 120.0), 0.0),
        (BeamColor::Green, Vec2::new(-500.0, 0.0), 12.0),
        (BeamColor::Blue, Vec2::new(-500.0, -120.0), 30.0),
    ];
    for (color, pivot, offset) in beams {
        let mut c = color.color();
        c.set_a(0.5);
        let material = beam_mats.add(BeamMaterial {
            color: c,
            time: 0.0,
            offset,
            selected: 0.0,
        });
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                transform: Transform::default()
                    .with_translation(Vec3::new(pivot.x + BEAM_LENGTH / 2.0, pivot.y, 0.0))
                    .with_scale(Vec3::new(BEAM_LENGTH, 40.0, 1.0)),
                material,
                ..default()
            })
            .insert(Pivot(pivot))
            .insert(color);
    }

    // player
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh.into(),
            transform: Transform::default()
                .with_translation(Vec3::new(-550.0, 0.0, 1.0))
                .with_scale(Vec3::splat(50.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Player);
}

// types and components

#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Pivot(Vec2);

#[derive(Component, PartialEq, Eq, Copy, Clone)]
pub enum BeamColor {
    Red,
    Green,
    Blue,
}

impl BeamColor {
    fn color(&self) -> Color {
        match self {
            BeamColor::Red => Color::rgb(1.0, 0.0, 0.0),
            BeamColor::Green => Color::rgb(0.0, 1.0, 0.0),
            BeamColor::Blue => Color::rgb(0.0, 0.0, 1.0),
        }
    }
}
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Killable {
    seconds: f32,
}

#[derive(Component, PartialEq, Eq, Copy, Clone)]
pub enum Colour {
    Red,
    Green,
    Blue,
    // red + green
    Yellow,
    // red + blue
    Magenta,
    // green + blue
    Cyan,
    // red + green + blue
    White,
}
const ALL_COLORS: [Colour; 7] = [
    Colour::Red,
    Colour::Green,
    Colour::Blue,
    Colour::Yellow,
    Colour::Magenta,
    Colour::Cyan,
    Colour::White,
];

impl Colour {
    fn made_by(&self) -> Vec<BeamColor> {
        use BeamColor::*;
        match self {
            Self::Red => vec![Red],
            Self::Green => vec![Green],
            Self::Blue => vec![Blue],
            Self::Yellow => vec![Red, Green],
            Self::Magenta => vec![Red, Blue],
            Self::Cyan => vec![Green, Blue],
            Self::White => vec![Red, Green, Blue],
        }
    }

    fn color(&self) -> Color {
        let mut c = self
            .made_by()
            .into_iter()
            .map(|c| c.color())
            .fold(Color::NONE, |a, b| a + b);
        c.set_a(1.0);
        c
    }
}
