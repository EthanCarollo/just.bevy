use bevy::{prelude::*};
use bevy::window::WindowResolution;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};
// Very good ressources overall for Bevy: https://bevy-cheatbook.github.io/introduction.html

mod et_scene;
use et_scene::base::ETScene;


fn main() {
    let window_resolution: WindowResolution =
        WindowResolution::new(800., 600.).with_scale_factor_override(1.0);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // The information for the Window Object from the Window Plugin can be found here :
                // https://docs.rs/bevy/latest/bevy/window/struct.Window.html
                resolution: window_resolution,
                title: "POC for Web Culture".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }).set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_plugins(PixelCameraPlugin)
        .add_systems(Update, update)
        .run();
}

fn update() {
    // classic update function
}

fn setup(commands: Commands, asset_server: Res<AssetServer>) {
    let _sprite: SpriteBundle = SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(5.0),
            ..Default::default()
        },
        texture: asset_server.load("test.png"),
        ..Default::default()
    };
    let _scene: ETScene = ETScene::new(commands)
        .add_entity((
            Camera2dBundle::default(),
            PixelZoom::FitSize {
                width: 320,
                height: 180,
            },
            PixelViewport,
        )).add_entity(_sprite);

}


