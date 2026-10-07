//! SPDX-License-Identifier: WTFPL

use bevy::prelude::*;

use crate::game::ui::{
    homescreen::{hover_thumbnails, scroll_main},
    homescreen_ui,
    palette::bg,
};

mod game;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Araria".into(),
                resolution: (1440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(bg()))
        .add_systems(Startup, setup)
        .add_systems(Update, (hover_thumbnails, scroll_main))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(bg()),
        ))
        .with_children(|root| {
            homescreen_ui(root);
        });
}
