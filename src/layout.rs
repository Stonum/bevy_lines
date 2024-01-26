use bevy::prelude::*;

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

#[derive(Component)]
pub struct Header;

#[derive(Component)]
pub struct HeaderLeft;

#[derive(Component)]
pub struct HeaderCenter;

#[derive(Component)]
pub struct HeaderRight;

#[derive(Component)]
pub struct Main;

#[derive(Component)]
pub struct Footer;

fn setup(mut commands: Commands) {
    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::percent(12.5),
                    GridTrack::flex(1.0),
                    GridTrack::percent(12.5),
                ],
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        align_items: AlignItems::Center,
                        // Make this node span two grid columns so that it takes up the entire top tow
                        grid_column: GridPlacement::span(1),
                        grid_template_columns: vec![GridTrack::flex(1.0); 3],
                        ..default()
                    },
                    ..default()
                })
                .insert(Header)
                .with_children(|builder| {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                grid_column: GridPlacement::span(1),
                                justify_content: JustifyContent::End,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(HeaderLeft);

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                grid_column: GridPlacement::span(1),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(HeaderCenter);

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                grid_column: GridPlacement::span(1),
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(HeaderRight);
                });

            // Main
            builder
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(Main);

            // Footer
            builder
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        grid_column: GridPlacement::span(1),
                        ..default()
                    },
                    ..default()
                })
                .insert(Footer);
        });
}
