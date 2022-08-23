use crate::*;

pub fn setup_menu(
    mut commands: Commands,
    a: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "a",
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    " bevy",
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 20.0,
                        color: Color::RED,
                    },
                ),
                TextSection::new(
                    " jam",
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 20.0,
                        color: Color::GREEN,
                    },
                ),
                TextSection::new(
                    " #002",
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 20.0,
                        color: Color::BLUE,
                    },
                ),
                TextSection::new(
                    " game",
                    TextStyle {
                        font: a.font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position: UiRect {
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            }),
        )
        .insert(CleanupMenu);

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
            // empty divs lol
            parent.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                ..default()
            });
            parent.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                ..default()
            });

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

    for (color, pos) in [
        (Color::WHITE, Vec3::new(0.0, 100.0, 2.0)),
        (Color::rgba(1.0, 0.0, 0.0, 0.7), Vec3::new(7.0, 103.0, 1.0)),
        (Color::rgba(0.0, 1.0, 0.0, 0.7), Vec3::new(1.0, 107.0, 1.0)),
        (Color::rgba(0.0, 0.0, 1.0, 0.7), Vec3::new(3.0, 101.0, 1.0)),
    ] {
        let mut a = commands.spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "luminity",
                TextStyle {
                    font: a.font.clone(),
                    font_size: 80.0,
                    color,
                },
            )
            .with_alignment(TextAlignment::TOP_CENTER),
            transform: Transform::from_translation(pos),
            ..default()
        });
        a.insert(CleanupMenu);
        if pos.z < 1.5 {
            a.insert(MenuTitle(pos.x, 100.0 - pos.y));
        }
    }

    let mesh = meshes.add(Mesh::from(shape::Circle::new(10.0)));
    for (i, color) in [Color::RED, Color::GREEN, Color::BLUE]
        .repeat(3)
        .into_iter()
        .enumerate()
    {
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: mesh.clone().into(),
                transform: Transform::default(),
                material: materials.add(ColorMaterial::from(color)),
                ..default()
            })
            .insert(MenuFloatingLight(i + 1))
            .insert(CleanupMenu);
    }
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

#[derive(Component)]
pub struct MenuFloatingLight(usize);

pub fn rotate_menu_lights(mut query: Query<(&mut Transform, &MenuFloatingLight)>, time: Res<Time>) {
    let t = time.seconds_since_startup();
    // make the friends go further when the button is pressed, but close in when activating a pillar
    for (mut trans, light) in query.iter_mut() {
        let i = light.0 as f64 / 2.0;
        let i2 = i / 2.0;
        let new_pos = Vec3::new(
            200.0 * i2 as f32 * (t * 0.4 * i2 + i).cos() as f32,
            200.0 * (t * 0.4 * i + i).sin() as f32,
            0.0,
        );

        trans.translation = new_pos;
    }
}

#[derive(Component)]
pub struct MenuTitle(f32, f32);

pub fn menu_title_parallax(mut query: Query<(&mut Transform, &MenuTitle)>, pos: Res<MousePos>) {
    for (mut trans, title) in &mut query {
        trans.translation.x = -title.0 * pos.pos.x / 100.0;
        trans.translation.y = 100.0 + title.1 * pos.pos.y / 100.0;
    }
}
