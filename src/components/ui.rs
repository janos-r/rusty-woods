use bevy::prelude::*;

#[derive(Component)]
pub struct TextBoxContainer;

#[derive(Component)]
pub struct TextBox;

pub fn spawn_children_text(
    font_handle: Handle<Font>,
    text: String,
) -> impl FnOnce(&mut ChildBuilder) {
    const FONT_SIZE: f32 = 30.;
    move |parent: &mut ChildBuilder| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: font_handle.clone(),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
        ));
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Bottom text box
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Px(200.),
            // align container to the bottom
            align_self: AlignSelf::FlexEnd,
            // makes space bellow the box
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // transparent container
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        // Debug - comment out invisibility on initial creation of the box to see it
        visibility: Visibility::Hidden,
        ..default()
    })
    .insert(TextBoxContainer)
    .with_children(|parent| {
        // box size, border thickness and color
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.),
            height: Val::Percent(80.),
                border: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            background_color: Color::MIDNIGHT_BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            // text background
            let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
            parent.spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    width: Val::Percent(100.),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .insert(TextBox)
            .insert(font_handle.clone())
            .with_children(spawn_children_text(font_handle, String::from(
                "Text Example a little longer trying to cross the width... Text Example a little longer trying to cross the width"
            )));
        });
    });
}
