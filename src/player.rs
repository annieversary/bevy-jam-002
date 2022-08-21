use crate::*;

pub fn move_player(
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

pub struct PlayerHealth {
    pub health: i8,
}

pub fn end_game_if_health_is_0(health: Res<PlayerHealth>) {
    if health.is_changed() && health.health <= 0 {
        // TODO end game
    }
}
