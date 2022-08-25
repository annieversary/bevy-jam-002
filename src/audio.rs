use crate::*;

pub fn play_audio_when_enemy_dies(
    a: Res<GameAssets>,
    enemy_death: EventReader<EnemyDead>,
    audio: Res<Audio>,
    mut idx: Local<usize>,
) {
    let melody = [
        &a.enemy_killed_sound_c,
        &a.enemy_killed_sound_g,
        &a.enemy_killed_sound_e,
        &a.enemy_killed_sound_d,
        &a.enemy_killed_sound_c,
        &a.enemy_killed_sound_e,
        &a.enemy_killed_sound_d,
        &a.enemy_killed_sound_f,
        &a.enemy_killed_sound_c,
        &a.enemy_killed_sound_e,
        &a.enemy_killed_sound_d,
        &a.enemy_killed_sound_g,
        &a.enemy_killed_sound_c,
        &a.enemy_killed_sound_f,
        &a.enemy_killed_sound_e,
        &a.enemy_killed_sound_d,
    ];

    if enemy_death.is_empty() {
        return;
    }
    enemy_death.clear();

    let music = melody[*idx % melody.len()].clone();
    audio.play(music);
    *idx += 1;
}
