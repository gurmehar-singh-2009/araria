use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::game::ui::homescreen::{
    GAMES, MainScroll, greeting, sidebar, spawn_friends, spawn_section, top_bar,
};

pub mod homescreen;
mod loading_screen;
pub mod palette;

pub fn homescreen_ui(root: &mut RelatedSpawnerCommands<'_, ChildOf>) {
    root.spawn(top_bar());

    root.spawn(Node {
        flex_grow: 1.0,
        min_height: px(0),
        ..default()
    })
    .with_children(|body| {
        body.spawn(sidebar());

        body.spawn((
            Node {
                flex_grow: 1.0,
                min_width: px(0),
                flex_direction: FlexDirection::Column,
                row_gap: px(28),
                padding: UiRect::all(px(24)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            MainScroll,
        ))
        .with_children(|main| {
            main.spawn(greeting());

            spawn_friends(main);

            spawn_section(main, "Continue", &GAMES[0..8]);
            spawn_section(main, "Recommended For You", &GAMES[8..16]);
            spawn_section(main, "Top Trending", &GAMES[2..10]);
            spawn_section(main, "Friends Are Playing", &GAMES[6..14]);
            spawn_section(main, "Popular", &GAMES[4..12]);
        });
    });
}
