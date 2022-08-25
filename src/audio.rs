use crate::*;

pub fn play_audio_when_enemy_dies(
    a: Res<AudioAssets>,
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

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "sounds/music.ogg")]
    pub music: Handle<AudioSource>,
    #[asset(path = "sounds/enemy-c.ogg")]
    enemy_killed_sound_c: Handle<AudioSource>,
    #[asset(path = "sounds/enemy-d.ogg")]
    enemy_killed_sound_d: Handle<AudioSource>,
    #[asset(path = "sounds/enemy-e.ogg")]
    enemy_killed_sound_e: Handle<AudioSource>,
    #[asset(path = "sounds/enemy-f.ogg")]
    enemy_killed_sound_f: Handle<AudioSource>,
    #[asset(path = "sounds/enemy-g.ogg")]
    enemy_killed_sound_g: Handle<AudioSource>,
}
