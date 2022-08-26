use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::{prelude::ThreadRng, seq::SliceRandom, thread_rng};

use crate::*;

#[derive(Deref, DerefMut)]
pub struct EnemySpawnerTimer(pub Timer);
pub fn spawn_enemies(
    mut cmd: Commands,
    pivots: Query<&Pivot>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut timer: ResMut<EnemySpawnerTimer>,
    time: Res<Time>,
    game_start: Res<GameStartTime>,
    mut mats: ResMut<Assets<EnemyMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    symbols: Res<EnemySymbols>,
    a: Res<EnemyAssets>,
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
    let c = get_random_colour(&mut rng, time.seconds_since_startup() - game_start.0);

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));
    let material = mats.add(EnemyMaterial {
        color: c.color(),
        time: 0.0,
        damaged: 0.0,
        symbol: match c {
            Colour::Red => symbols.red.clone(),
            Colour::Green => symbols.green.clone(),
            Colour::Blue => symbols.blue.clone(),
            Colour::Yellow => symbols.yellow.clone(),
            Colour::Magenta => symbols.magenta.clone(),
            Colour::Cyan => symbols.cyan.clone(),
            Colour::White => symbols.white.clone(),
        },
    });
    cmd.spawn_bundle(SpriteSheetBundle {
        transform: Transform::default()
            .with_translation(pivot.extend(1.0))
            .with_scale(Vec3::splat(2.0)),
        texture_atlas: a.enemy.clone(),
        ..default()
    })
    .insert(Enemy)
    .insert(Killable {
        seconds: 0.0,
        under_damage: false,
    })
    .insert(CleanupGame)
    .insert(c)
    .insert(AnimationTimer {
        timer: Timer::from_seconds(0.1, true),
        paused: false,
    })
    .with_children(|parent| {
        parent.spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh.into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, 0.0, 2.0))
                .with_scale(Vec3::new(15., 15.0, 1.0)),
            material,
            ..default()
        });
    });
}

pub fn get_random_colour(rng: &mut ThreadRng, time: f64) -> Colour {
    if time < 20.0 {
        return *[Colour::Red, Colour::Green, Colour::Blue]
            .choose(rng)
            .unwrap();
    }
    if time < 40.0 {
        return *[
            Colour::Red,
            Colour::Green,
            Colour::Blue,
            Colour::Red,
            Colour::Green,
            Colour::Blue,
            Colour::Cyan,
            Colour::Magenta,
            Colour::Yellow,
        ]
        .choose(rng)
        .unwrap();
    }
    if time < 60.0 {
        return *[
            Colour::Red,
            Colour::Green,
            Colour::Blue,
            Colour::Cyan,
            Colour::Magenta,
            Colour::Yellow,
        ]
        .choose(rng)
        .unwrap();
    }
    *ALL_COLORS.choose(rng).unwrap()
}

pub fn move_enemies(
    mut query: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let dt = time.delta_seconds();
    let player = player.single();
    for mut trans in &mut query {
        if trans.translation.x > -500.0 {
            trans.translation.x -= dt * 50.0;
        } else if let Some(v) = (player.translation.xy() - trans.translation.xy()).try_normalize() {
            let v = v * 50.0 * dt;
            trans.translation.x += v.x;
            trans.translation.y += v.y;
        }
    }
}

pub struct EnemiesKilled(pub usize);
pub struct EnemyDead;

pub fn damage_enemies(
    mut cmd: Commands,
    mut killable: Query<(Entity, &Transform, &Colour, &mut Killable)>,
    beams: Query<(&Transform, &Pivot, &BeamColor)>,
    time: Res<Time>,
    mut killed: ResMut<EnemiesKilled>,
    mut ev: EventWriter<EnemyDead>,
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

        killable.under_damage = false;

        // if any of the required colors is not hitting, exit
        for c in colour.made_by() {
            if !hitting_colors.contains(&c) {
                continue 'ent;
            }
        }

        killable.under_damage = true;
        killable.seconds += time.delta_seconds();

        if killable.seconds > 2.0 {
            cmd.entity(entity).despawn_recursive();
            killed.0 += 1;
            ev.send(EnemyDead);
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

pub fn damage_player(
    mut cmd: Commands,
    player: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut health: ResMut<PlayerHealth>,
) {
    let player = player.single();
    for (entity, trans) in &enemies {
        if player.translation.xy().distance(trans.translation.xy()) < 55.0 {
            cmd.entity(entity).despawn_recursive();
            health.health -= 1;
        }
    }
}

impl Material2d for EnemyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/enemy_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "999abc99-d598-45ab-8225-97e2a3f056e0"]
pub struct EnemyMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub damaged: f32,
    #[texture(1)]
    #[sampler(2)]
    symbol: Handle<Image>,
}

pub fn update_enemy_material(
    mut parents: Query<(&mut TextureAtlasSprite, &Killable)>,
    children: Query<(&Handle<EnemyMaterial>, &Parent)>,
    mut a: ResMut<Assets<EnemyMaterial>>,
    time: Res<Time>,
) {
    for (handle, parent) in &children {
        if let Some(mat) = a.get_mut(handle) {
            mat.time = time.seconds_since_startup() as f32;
            if let Ok((_, killable)) = parents.get(**parent) {
                mat.damaged = killable.under_damage.then_some(1.0).unwrap_or_default();
            }
        }
    }

    for (mut atlas, killable) in &mut parents {
        let s = (time.seconds_since_startup() * 6.0).sin() * 0.2 + 0.8;
        let a = killable.under_damage.then_some(s).unwrap_or(1.0);
        atlas.color.set_a(a as f32);
    }
}

#[derive(AssetCollection)]
pub struct EnemySymbols {
    #[asset(path = "sprites/symbols/red.png")]
    red: Handle<Image>,
    #[asset(path = "sprites/symbols/green.png")]
    green: Handle<Image>,
    #[asset(path = "sprites/symbols/blue.png")]
    blue: Handle<Image>,
    #[asset(path = "sprites/symbols/yellow.png")]
    yellow: Handle<Image>,
    #[asset(path = "sprites/symbols/magenta.png")]
    magenta: Handle<Image>,
    #[asset(path = "sprites/symbols/cyan.png")]
    cyan: Handle<Image>,
    #[asset(path = "sprites/symbols/white.png")]
    white: Handle<Image>,
}

pub struct EnemyAssets {
    pub enemy: Handle<TextureAtlas>,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut textures = cell.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        let game_assets = cell
            .get_resource::<GameAssets>()
            .expect("Failed to get GameAssets");

        let enemy = textures.add(TextureAtlas::from_grid(
            game_assets.enemy.clone(),
            Vec2::new(48.0, 48.0),
            2,
            1,
        ));
        Self { enemy }
    }
}
