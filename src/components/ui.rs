use bevy::prelude::*;

#[derive(Component)]
pub struct TextBoxContainer;

#[derive(Component)]
pub struct TextBox;

pub fn spawn_children_text(
    font_handle: Handle<Font>,
    text: String,
) -> impl FnOnce(&mut ChildBuilder) {
    // Known issue: text wrapping solution (bug workaround) based on: https://github.com/bevyengine/bevy/issues/1490
    const FONT_SIZE: f32 = 30.;
    move |parent: &mut ChildBuilder| {
        // "Text Example",
        for word in text.split_whitespace() {
            parent.spawn(
                TextBundle::from_section(
                    word.to_string(),
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    // this is required because of the bevy bug https://github.com/bevyengine/bevy/issues/5834
                    max_size: Size::new(Val::Undefined, Val::Px(FONT_SIZE)),
                    // this is the size of the spaces between words
                    margin: UiRect::all(Val::Px(4.)),
                    ..default()
                }),
            );
        }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Bottom text box
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
            // align container to the bottom
            align_self: AlignSelf::FlexEnd,
            // makes space bellow the box
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // transparent container
        background_color: Color::rgba(0.65, 0.65, 0.65, 0.).into(),
        ..default()
    })
    .insert(TextBoxContainer)
    .with_children(|parent| {
        // box size, border thickness and color
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.), Val::Percent(80.0)),
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
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                    padding: UiRect::all(Val::Px(6.)),
                    // don't stretch vertically
                    align_content: AlignContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .insert(TextBox)
            .insert(font_handle.clone())
            .with_children(spawn_children_text(font_handle, String::from(
                // "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularized in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
                "Text Example a little longer trying to cross the width. Text Example a little longer trying to cross the width"
            )));
        });
    });
}
