use crate::*;

pub fn setup_ui(mut cmd: Commands, a: Res<GameAssets>) {
    cmd.spawn_bundle(
        TextBundle::from_section(
            "health: 30",
            TextStyle {
                font: a.font.clone(),
                font_size: 80.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    )
    .insert(CleanupGame)
    .insert(PlayerHealthText);
    cmd.spawn_bundle(
        TextBundle::from_section(
            "points: 0",
            TextStyle {
                font: a.font.clone(),
                font_size: 80.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    )
    .insert(CleanupGame)
    .insert(PointsText);
}

#[derive(Component)]
pub struct PlayerHealthText;
pub fn update_player_health_ui(
    health: Res<PlayerHealth>,
    mut texts: Query<&mut Text, With<PlayerHealthText>>,
) {
    if !health.is_changed() {
        return;
    }

    for mut text in &mut texts {
        // Update the color of the first and only section.
        text.sections[0].value = format!("health: {}", health.health);
    }
}

#[derive(Component)]
pub struct PointsText;
pub fn update_points_ui(points: Res<EnemiesKilled>, mut texts: Query<&mut Text, With<PointsText>>) {
    if !points.is_changed() {
        return;
    }

    for mut text in &mut texts {
        // Update the color of the first and only section.
        text.sections[0].value = format!("points: {}", points.0);
    }
}
