#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_asset_loader::prelude::*;

mod audio;
mod beams;
mod death_screen;
mod enemies;
mod menu;
mod mouse;
mod player;
mod ui;

use audio::*;
use beams::*;
use death_screen::*;
use enemies::*;
use menu::*;
use mouse::*;
use player::*;
use ui::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Menu,
    Game,
    Death,
}

pub const BEAM_LENGTH: f32 = 1000.0;
pub const ENEMY_RADIUS: f32 = 50.0;
pub const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Loading)
        .add_plugin(Material2dPlugin::<BeamMaterial>::default())
        .add_plugin(Material2dPlugin::<EnemyMaterial>::default())
        // .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .with_collection::<AudioAssets>()
                .with_collection::<GameAssets>()
                .with_collection::<EnemySymbols>()
                .init_resource::<PlayerAssets>()
                .init_resource::<EnemyAssets>(),
        )
        .init_resource::<MousePos>()
        .insert_resource(EnemiesKilled(0))
        .insert_resource(GameStartTime(0.0))
        .insert_resource(ClosestBeam(BeamColor::Green))
        .insert_resource(EnemySpawnerTimer(Timer::from_seconds(1.0, true)))
        .insert_resource(PlayerHealth { health: 30 })
        .add_event::<EnemyDead>()
        .add_system(button_interaction)
        .add_system(update_mouse_pos)
        .add_system(animate_sprite)
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup))
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
        .add_system_set(
            SystemSet::on_update(GameState::Menu)
                .with_system(menu)
                .with_system(rotate_menu_lights)
                .with_system(menu_title_parallax),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup::<CleanupMenu>))
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(game_setup)
                .with_system(setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(move_player)
                .with_system(end_game_if_health_is_0)
                .with_system(change_player_sprite)
                .with_system(move_light_beam)
                .with_system(update_beam_material)
                .with_system(update_closest_beam)
                .with_system(spawn_enemies)
                .with_system(move_enemies)
                .with_system(damage_enemies)
                .with_system(play_audio_when_enemy_dies)
                .with_system(damage_player)
                .with_system(update_enemy_material)
                .with_system(update_player_health_ui)
                .with_system(update_points_ui),
        )
        .add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup::<CleanupGame>))
        .add_system_set(SystemSet::on_enter(GameState::Death).with_system(setup_death_screen))
        .add_system_set(SystemSet::on_update(GameState::Death).with_system(death_screen))
        .add_system_set(SystemSet::on_exit(GameState::Death).with_system(cleanup::<CleanupDeath>))
        .run();
}

fn setup(mut commands: Commands, a: Res<AssetServer>, aa: Res<AudioAssets>, audio: Res<Audio>) {
    a.watch_for_changes().unwrap();

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    audio.play_with_settings(aa.music.clone(), PlaybackSettings::LOOP);
}

pub struct GameStartTime(f64);

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut beam_mats: ResMut<Assets<BeamMaterial>>,

    mut health: ResMut<PlayerHealth>,
    mut score: ResMut<EnemiesKilled>,
    mut start: ResMut<GameStartTime>,
    time: Res<Time>,
    a: Res<PlayerAssets>,
) {
    // reset resources
    health.health = 30;
    score.0 = 0;
    start.0 = time.seconds_since_startup();

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
            .insert(color)
            .insert(CleanupGame);
    }

    // player
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::default().with_translation(Vec3::new(-550.0, 0.0, 1.0)),
            texture_atlas: a.player.clone(),
            ..default()
        })
        .insert(CleanupGame)
        .insert(Player)
        .insert(AnimationTimer {
            timer: Timer::from_seconds(0.1, true),
            paused: false,
        });
}

//

#[derive(Component)]
pub struct CleanupMenu;
#[derive(Component)]
pub struct CleanupGame;
#[derive(Component)]
pub struct CleanupDeath;
pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
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
    under_damage: bool,
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

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fonts/gameplay.ttf")]
    font: Handle<Font>,
    #[asset(path = "sprites/player.png")]
    player: Handle<Image>,
    #[asset(path = "sprites/player-neutral.png")]
    player_neutral: Handle<Image>,
    #[asset(path = "sprites/player-sad.png")]
    player_sad: Handle<Image>,
    #[asset(path = "sprites/enemy.png")]
    enemy: Handle<Image>,
}

pub fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
pub struct AnimationTimer {
    timer: Timer,
    paused: bool,
}
fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        if timer.paused {
            sprite.index = 0;
        } else {
            timer.timer.tick(time.delta());
            if timer.timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }
}
