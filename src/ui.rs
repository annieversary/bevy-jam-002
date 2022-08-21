use crate::*;

pub fn setup_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/gameplay.ttf");

    cmd.spawn_bundle(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "health: 30",
            TextStyle {
                font,
                font_size: 80.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
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
    .insert(PlayerHealthText);
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
