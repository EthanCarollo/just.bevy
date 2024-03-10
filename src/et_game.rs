use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, ImagePlugin, Res, Window, WindowPlugin};
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
}