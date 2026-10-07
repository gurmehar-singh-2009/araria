use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::game::ui::palette::{
    field, like_green, muted, muted_2, panel, player_blue, selected, text,
};

pub struct Game {
    title:   &'static str,
    rating:  u8,
    players: &'static str,
}

pub const GAMES: &[Game] = &[
    Game {
        title:   "Brookhaven Town",
        rating:  87,
        players: "412K",
    },
    Game {
        title:   "Blade Ball",
        rating:  92,
        players: "238K",
    },
    Game {
        title:   "Adopt a Pet",
        rating:  89,
        players: "176K",
    },
    Game {
        title:   "Tower of Hell",
        rating:  81,
        players: "64K",
    },
    Game {
        title:   "Murder Mystery 3",
        rating:  90,
        players: "52K",
    },
    Game {
        title:   "Obby But You're on a Bike",
        rating:  85,
        players: "48K",
    },
    Game {
        title:   "Pet Simulator Deluxe",
        rating:  94,
        players: "41K",
    },
    Game {
        title:   "Natural Disaster Survival",
        rating:  88,
        players: "37K",
    },
    Game {
        title:   "Jailbreak",
        rating:  86,
        players: "33K",
    },
    Game {
        title:   "Work at a Pizza Place",
        rating:  91,
        players: "29K",
    },
    Game {
        title:   "Doors",
        rating:  95,
        players: "27K",
    },
    Game {
        title:   "Arsenal",
        rating:  90,
        players: "22K",
    },
    Game {
        title:   "Fisch",
        rating:  93,
        players: "19K",
    },
    Game {
        title:   "Dungeon Quest",
        rating:  87,
        players: "14K",
    },
    Game {
        title:   "Super Golf!",
        rating:  84,
        players: "9.8K",
    },
    Game {
        title:   "Lumber Tycoon 2",
        rating:  83,
        players: "7.1K",
    },
];

const FRIENDS: &[&str] = &[
    "Alex", "Bri", "Casper", "Dana", "Eli", "Fern", "Gus", "Hana", "Ivo", "Jun",
];

#[derive(Component)]
pub struct GameThumb;

#[derive(Component)]
pub struct MainScroll;

pub fn top_bar() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: px(48),
            align_items: AlignItems::Center,
            padding: UiRect::axes(px(16), px(0)),
            column_gap: px(24),
            flex_shrink: 0.0,
            ..default()
        },
        BackgroundColor(panel()),
        children![
            label("Araria", 22.0, text()),
            label("Games", 15.0, text()),
            label("Players", 15.0, muted()),
            label("Create", 15.0, muted()),
            // Search box
            (
                Node {
                    width: px(380),
                    height: px(32),
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(px(12), px(0)),
                    border_radius: BorderRadius::all(px(8)),
                    ..default()
                },
                BackgroundColor(field()),
                children![label("Search", 14.0, muted())],
            ),
            // Spacer
            Node {
                flex_grow: 1.0,
                ..default()
            },
            label("6,767 Robux", 14.0, text()),
        ],
    )
}

pub fn sidebar() -> impl Bundle {
    (
        Node {
            width: px(240),
            flex_shrink: 0.0,
            flex_direction: FlexDirection::Column,
            row_gap: px(4),
            padding: UiRect::all(px(12)),
            ..default()
        },
        BackgroundColor(panel()),
        children![
            nav_item("Home", true),
            nav_item("Profile", false),
            nav_item("Inbox", false),
        ],
    )
}

fn nav_item(name: &str, active: bool) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: px(40),
            align_items: AlignItems::Center,
            padding: UiRect::axes(px(12), px(0)),
            border_radius: BorderRadius::all(px(8)),
            ..default()
        },
        BackgroundColor(if active { selected() } else { Color::NONE }),
        children![label(name, 16.0, if active { text() } else { muted() })],
    )
}

pub fn greeting() -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            column_gap: px(16),
            ..default()
        },
        children![
            avatar(64.0, "A", Color::hsl(210.0, 0.5, 0.45)),
            label("Hello, Araria Player!", 28.0, text()),
        ],
    )
}

pub fn spawn_friends(main: &mut ChildSpawnerCommands) {
    main.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(12),
        ..default()
    })
    .with_children(|section| {
        section.spawn(label(&format!("Friends ({})", FRIENDS.len()), 22.0, text()));
        section
            .spawn(Node {
                column_gap: px(16),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|row| {
                for (i, name) in FRIENDS.iter().enumerate() {
                    let initial = name.chars().next().unwrap_or('?').to_string();
                    row.spawn((
                        Node {
                            width: px(76),
                            flex_shrink: 0.0,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: px(6),
                            ..default()
                        },
                        children![
                            avatar(72.0, &initial, muted_2()),
                            label(name, 13.0, muted()),
                        ],
                    ));
                }
            });
    });
}

pub fn spawn_section(main: &mut ChildSpawnerCommands, title: &str, games: &[Game]) {
    main.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: px(12),
        ..default()
    })
    .with_children(|section| {
        section.spawn(label(title, 22.0, text()));
        section
            .spawn(Node {
                column_gap: px(12),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|row| {
                for game in games {
                    row.spawn(game_card(game));
                }
            });
    });
}

fn game_card(game: &Game) -> impl Bundle {
    let hue = game
        .title
        .bytes()
        .fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32))
        % 360;
    let initial = game.title.chars().next().unwrap_or('?').to_string();

    (
        Node {
            width: px(170),
            flex_shrink: 0.0,
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(6),
            ..default()
        },
        children![
            // Game icon.
            (
                Button,
                GameThumb,
                Node {
                    width: percent(100),
                    aspect_ratio: Some(1.0),
                    border: UiRect::all(px(2)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border_radius: BorderRadius::all(px(8)),
                    ..default()
                },
                BorderColor::all(Color::NONE),
                BackgroundColor(muted_2()),
                // children![(
                //     Text::new(initial),
                //     TextFont {
                //         font_size: FontSize::Px(56.),
                //         ..default()
                //     },
                //     TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
                // )],
            ),
            // Title
            (
                Node {
                    height: px(36),
                    overflow: Overflow::clip(),
                    ..default()
                },
                children![label(game.title, 14.0, text())],
            ),
            // Stats
            (
                Node {
                    align_items: AlignItems::Center,
                    column_gap: px(16),
                    ..default()
                },
                children![
                    stat(like_green(), format!("{}%", game.rating)),
                    stat(player_blue(), game.players.to_string()),
                ],
            ),
        ],
    )
}

fn stat(color: Color, value: String) -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            column_gap: px(4),
            ..default()
        },
        children![
            (
                Node {
                    width: px(8),
                    height: px(8),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BackgroundColor(color),
            ),
            label(&value, 12.0, muted()),
        ],
    )
}

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s),
        TextFont {
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    )
}

fn avatar(size: f32, initial: &str, color: Color) -> impl Bundle {
    (
        Node {
            width: px(size),
            height: px(size),
            flex_shrink: 0.0,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BackgroundColor(color),
        // children![(
            // Text::new(initial),
            // TextFont {
                // font_size: FontSize::Px(size * 0.45),
                // ..default()
            // },
            // TextColor(text()),
        // )],
    )
}

pub fn hover_thumbnails(
    mut q: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<GameThumb>)>,
) {
    for (interaction, mut border) in &mut q {
        *border = match interaction {
            Interaction::Hovered => BorderColor::all(Color::WHITE),
            Interaction::Pressed => BorderColor::all(Color::srgb_u8(255, 0, 0)),
            Interaction::None => BorderColor::all(Color::NONE),
        };
    }
}

pub fn scroll_main(
    mut wheel: MessageReader<MouseWheel>,
    mut q: Query<(&mut ScrollPosition, &ComputedNode), With<MainScroll>>,
) {
    let mut dy = 0.0;
    for ev in wheel.read() {
        dy += match ev.unit {
            MouseScrollUnit::Line => ev.y * 40.0,
            MouseScrollUnit::Pixel => ev.y,
        };
    }
    if dy == 0.0 {
        return;
    }

    for (mut pos, node) in &mut q {
        let max = ((node.content_size().y - node.size().y) * node.inverse_scale_factor()).max(0.0);
        pos.y = (pos.y - dy).clamp(0.0, max);
    }
}
