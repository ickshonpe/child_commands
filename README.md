# Child Commands

Experimental extension trait / facade on EntityCommands for more ergonomic 
entity hierarchy spawning.

Superseded by flat_commands, extending EntityCommands was a silly idea.

## Examples

### Before:

```rust
fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(1.0, 1.0, 1.0),
                    ..Default::default()
                })
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(2.0, 2.0, 2.0),
                    ..Default::default()
                })
        });
}
```

### or

```rust
fn setup(mut commands: Commands) {
    let child_1 = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .id();
    let child_2 = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0),
            ..Default::default()
        })
        .id();
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .push_child(child_1)
        .push_child(child_1);
}
```

### After:

```rust
use child_commands::*;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .with_child_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .with_sibling_bundle(PbrBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0),
            ..Default::default()
        });
}
```

### Before:

```rust
pub fn spawn_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
 
    commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn_bundle(NodeBundle {
                color: UiColor (Color::DARK_GRAY),
                style: Style {
                    padding: Rect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Hello, world!",
                    TextStyle {
                        font: asset_server.load("FiraMono-Regular.ttf"),
                        font_size: 16.0,
                        color: Color::ANTIQUE_WHITE,
                    },
                    TextAlignment::default()
                ),
                 ..Default::default()
            });
        });
    });
}
```

### After:

```rust
use child_commands::*;

pub fn spawn_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_child_bundle(NodeBundle {
        color: UiColor (Color::DARK_GRAY),
        style: Style {
            padding: Rect::all(Val::Px(4.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_child_bundle(TextBundle {
        text: Text::with_section(
            "Hello, world!",
            TextStyle {
                font: asset_server.load("FiraMono-Regular.ttf"),
                font_size: 16.0,
                color: Color::ANTIQUE_WHITE,
            },
            TextAlignment::default()
        ),
        ..Default::default()
    });
}
```

### Also have with_children for split hierarchies 

```rust
fn spawn_hierachy(
    mut commands: Commands
) {
    commands
    .spawn()     
    .with_child()
    .with_children(|builder| {
        builder
        .spawn()        // only an EntityCommands, so can't call with_sibling here unfortunately.
        .with_child()
        .with_sibling();
    })
    .with_sibling()
    .with_children(|builder| {
        builder
        .spawn()
        .with_child()
        .with_sibling();
    })
    .with_sibling()
    .with_children(|builder| {
        builder
        .spawn()
        .with_child()
        .with_sibling()
        .with_sibling()
        .with_sibling();
    });
}

```


## Other Info

* Used unsafe to hack around the private fields to get with_children on ChildCommands. *Very likely unsound.*

* The with_children builder returns an EntityCommands not a ChildCommands, despite it being a child entity.
I can't see a simple way with extension traits to get past the private fields in ChildBuilder and Children etc.

* No idea about performance.

## Todo

* Maybe some sort of spawn_brood function to spawn multiple children at once.

* Investigate if there is an ergonomic way to retrieve ids from the middle of the hierachy. "with_id" doesn't seem that promising.
Might be an anti-feature anyway. 



