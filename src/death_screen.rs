use crate::*;

pub fn setup_death_screen(mut commands: Commands, a: Res<GameAssets>, score: Res<EnemiesKilled>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(CleanupDeath)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "you died!",
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 80.0,
                        color: Color::RED,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER),
            );

            parent.spawn_bundle(
                TextBundle::from_section(
                    &format!("Points: {}", score.0),
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_CENTER),
            );

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(PlayAgainButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Play again!",
                        TextStyle {
                            font: a.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

#[derive(Component)]
pub struct PlayAgainButton;

pub fn death_screen(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayAgainButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            state.set(GameState::Game).unwrap();
        }
    }
}
