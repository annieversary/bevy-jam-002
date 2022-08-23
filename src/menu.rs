use crate::*;

pub fn setup_menu(mut commands: Commands, a: Res<GameAssets>) {
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
        .insert(CleanupMenu)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::new(
                        "bevy",
                        TextStyle {
                            font: a.font.clone(),
                            font_size: 80.0,
                            color: Color::RED,
                        },
                    ),
                    TextSection::new(
                        " jam",
                        TextStyle {
                            font: a.font.clone(),
                            font_size: 80.0,
                            color: Color::GREEN,
                        },
                    ),
                    TextSection::new(
                        " #002",
                        TextStyle {
                            font: a.font.clone(),
                            font_size: 80.0,
                            color: Color::BLUE,
                        },
                    ),
                ])
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
                .insert(PlayButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Play",
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
pub struct PlayButton;

pub fn menu(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            state.set(GameState::Game).unwrap();
        }
    }
}
