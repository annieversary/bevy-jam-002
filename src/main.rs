use bevy::{
    math::Vec3Swizzles, prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle,
};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .init_resource::<MousePos>()
        .insert_resource(ClosestBeam(BeamColor::Green))
        .insert_resource(EnemySpawnerTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(move_light_beam)
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));

    let beams = [
        (BeamColor::Red, Vec2::new(-500.0, 120.0)),
        (BeamColor::Green, Vec2::new(-500.0, 0.0)),
        (BeamColor::Blue, Vec2::new(-500.0, -120.0)),
    ];
    for (color, pivot) in beams {
        let mut c = color.color();
        c.set_a(0.5);
        let material = materials.add(ColorMaterial::from(c));
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

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Pivot(Vec2);

#[derive(Component, PartialEq, Eq, Copy, Clone)]
enum BeamColor {
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
struct Enemy;
#[derive(Component)]
struct Killable {
    seconds: f32,
}

#[derive(Component, PartialEq, Eq, Copy, Clone)]
enum Colour {
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

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut diff = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        diff += Vec2::Y;
    }
    if input.pressed(KeyCode::S) {
        diff -= Vec2::Y;
    }
    if input.pressed(KeyCode::A) {
        diff -= Vec2::X;
    }
    if input.pressed(KeyCode::D) {
        diff += Vec2::X;
    }
    diff *= 150.0 * time.delta_seconds();

    for mut trans in &mut query {
        trans.translation.y = (diff.y + trans.translation.y).clamp(-120.0, 140.0);
        trans.translation.x = (diff.x + trans.translation.x).clamp(-600.0, -525.0);
    }
}

struct ClosestBeam(BeamColor);
fn update_closest_beam(
    player: Query<&Transform, (With<Player>, Without<BeamColor>)>,
    beams: Query<(&Pivot, &BeamColor)>,
    mut color: ResMut<ClosestBeam>,
) {
    let pos = player.single().translation.xy();
    let mut beams = beams.iter().collect::<Vec<_>>();
    beams.sort_unstable_by(|a, b| {
        b.0 .0
            .distance(pos)
            .partial_cmp(&a.0 .0.distance(pos))
            .unwrap()
    });
    color.0 = *beams.pop().unwrap().1;
}

fn move_light_beam(
    mut query: Query<(&mut Transform, &Pivot, &BeamColor)>,
    color: Res<ClosestBeam>,
    mouse: Res<MousePos>,
) {
    for (mut trans, pivot, beam) in &mut query {
        if *beam == color.0 {
            //
            let diff =
                Vec2::new((mouse.pos.x - pivot.0.x).abs(), mouse.pos.y - pivot.0.y).normalize();
            let angle = diff.angle_between(Vec2::X);

            trans.translation = (pivot.0 + diff * BEAM_LENGTH / 2.0).extend(0.0);
            trans.rotation = Quat::from_rotation_z(-angle);
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct EnemySpawnerTimer(Timer);
fn spawn_enemies(
    mut cmd: Commands,
    pivots: Query<&Pivot>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut timer: ResMut<EnemySpawnerTimer>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let mut rng = thread_rng();

    let (camera, camera_transform) = q_camera.single();
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let spawn_x = ndc_to_world.project_point3(Vec3::new(1.0, 0.0, -1.0)).x + 50.0;

    // choose pivot
    let pivots = pivots.iter().collect::<Vec<_>>();
    let pivot = Vec2::new(spawn_x, pivots.choose(&mut rng).unwrap().0.y);
    // choose color
    let c = get_random_colour(&mut rng, time.seconds_since_startup());

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));
    cmd.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh.into(),
        transform: Transform::default()
            .with_translation(pivot.extend(1.0))
            .with_scale(Vec3::new(40., 40.0, 1.0)),
        material: materials.add(ColorMaterial::from(c.color())),
        ..default()
    })
    .insert(Enemy)
    .insert(Killable { seconds: 0.0 })
    .insert(c);
}

fn get_random_colour(rng: &mut ThreadRng, time: f64) -> Colour {
    if time < 20.0 {
        return *[Colour::Red, Colour::Green, Colour::Blue]
            .choose(rng)
            .unwrap();
    }

    *ALL_COLORS.choose(rng).unwrap()
}

fn move_enemies(mut query: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for mut trans in &mut query {
        trans.translation.x -= dt * 50.0;
    }
}

fn damage_enemies(
    mut cmd: Commands,
    mut killable: Query<(Entity, &Transform, &Colour, &mut Killable)>,
    beams: Query<(&Transform, &Pivot, &BeamColor)>,
    time: Res<Time>,
) {
    'ent: for (entity, trans, colour, mut killable) in &mut killable {
        // get the beams currently hitting the enemy
        let mut hitting_colors = vec![];
        for (beam_trans, pivot, color) in &beams {
            if is_intersect(
                beam_trans.translation.xy(),
                pivot.0,
                trans.translation.xy(),
                ENEMY_RADIUS,
            ) && trans.translation.xy().distance(pivot.0) < BEAM_LENGTH + ENEMY_RADIUS / 2.0
            {
                hitting_colors.push(*color);
            }
        }

        // if any of the required colors is not hitting, exit
        for c in colour.made_by() {
            if !hitting_colors.contains(&c) {
                continue 'ent;
            }
        }

        killable.seconds += time.delta_seconds();

        if killable.seconds > 2.0 {
            cmd.entity(entity).despawn_recursive();
        }
    }
}

fn is_intersect(line_a: Vec2, line_b: Vec2, circle_center: Vec2, circle_radius: f32) -> bool {
    let distance = ((line_b.x - line_a.x) * (line_a.y - circle_center.y)
        - (line_a.x - circle_center.x) * (line_b.y - line_a.y))
        .abs()
        / ((line_b.x - line_a.x).powi(2) + (line_b.y - line_a.y).powi(2)).sqrt();
    distance < circle_radius
}

#[derive(Default)]
struct MousePos {
    pos: Vec2,
}
fn update_mouse_pos(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse: ResMut<MousePos>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        mouse.pos = world_pos.truncate();
    }
}
