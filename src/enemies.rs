use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::*;

#[derive(Deref, DerefMut)]
pub struct EnemySpawnerTimer(pub Timer);
pub fn spawn_enemies(
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

pub fn get_random_colour(rng: &mut ThreadRng, time: f64) -> Colour {
    if time < 20.0 {
        return *[Colour::Red, Colour::Green, Colour::Blue]
            .choose(rng)
            .unwrap();
    }

    *ALL_COLORS.choose(rng).unwrap()
}

pub fn move_enemies(mut query: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for mut trans in &mut query {
        trans.translation.x -= dt * 50.0;
    }
}

pub fn damage_enemies(
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
