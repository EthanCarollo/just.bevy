use bevy::asset::AssetServer;
use bevy::prelude::{BuildChildren, ButtonBundle, Camera2dBundle, Color, Commands, NodeBundle, Res, TextBundle};
use bevy::text::TextStyle;
use bevy::ui::{AlignItems, BorderColor, JustifyContent, Style, UiRect, Val};
use bevy::utils::default;
use crate::et_scene::r#trait::ETScene;


pub struct ETMainMenu<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    PRESSED_BUTTON: Color,
    NORMAL_BUTTON: Color,
    HOVERED_BUTTON: Color
}

impl<'w, 's>   ETMainMenu<'w, 's> {
    pub fn new(_commands: Commands<'w, 's>, _asset_server: Res<'w, AssetServer>) -> ETMainMenu<'w, 's> {
        ETMainMenu{
            commands: _commands,
            asset_server: _asset_server,
            NORMAL_BUTTON: Color::rgb(0.15, 0.15, 0.15),
            HOVERED_BUTTON: Color::rgb(0.25, 0.25, 0.25),
            PRESSED_BUTTON: Color::rgb(0.35, 0.75, 0.35)
        }
    }

}

impl<'w, 's>  ETScene<'w, 's>  for ETMainMenu<'w, 's> {
    fn build_scene(&mut self) {
        // ui camera
        self.commands.spawn(Camera2dBundle::default());
        self.commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: self.NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font: self.asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            });
        println!("Hello World");
    }
}


