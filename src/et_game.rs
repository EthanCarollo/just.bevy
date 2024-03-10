use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_pixel_camera::{
    PixelCameraPlugin
};
use crate::et_scene::et_main_menu::ETMainMenu;

// traits function can only be used if the traits has been imported
use crate::et_scene::r#trait::ETScene;
// Very good ressources overall for Bevy:

pub struct ETGame<'a>{
    app: &'a mut App
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl<'a> ETGame<'a>{
    pub fn new( _app: &'a mut App) -> ETGame<'a> {

        let window_resolution: WindowResolution =
            WindowResolution::new(800., 600.).with_scale_factor_override(1.0);

        return ETGame{
            app: _app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // The information for the Window Object from the Window Plugin can be found here :
                    // https://docs.rs/bevy/latest/bevy/window/struct.Window.html
                    resolution: window_resolution,
                    title: "POC for Web Culture".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }).set(ImagePlugin::default_nearest()))
                .add_systems(Startup, ETGame::setup)
                .add_systems(Update, ETGame::button_system)
                .add_plugins(PixelCameraPlugin)
        };
    }

    pub fn run(&mut self){
        self.app.run();
    }

    pub fn setup(commands: Commands, asset_server: Res<AssetServer>){
        let mut scene = ETMainMenu::new(commands, asset_server);
        scene.build_scene();
    }

    // The fact that we can update the button created in ETMainMenu without any pointer to
    // really brainfuck me like, HOW ? (yes in fact its pretty simple but it's funny to see
    // this in low level language)
    pub fn button_system(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut Text>,
    ) {
        for (interaction, mut color, mut border_color, children) in &mut interaction_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Pressed => {
                    text.sections[0].value = "Press".to_string();
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = Color::RED;
                }
                Interaction::Hovered => {
                    text.sections[0].value = "Hover".to_string();
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    text.sections[0].value = "Button".to_string();
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}