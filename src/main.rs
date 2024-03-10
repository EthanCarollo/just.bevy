use bevy::{prelude::*};
use bevy::render::texture::ImageSampler;
use bevy::window::WindowResolution;
use bevy_pixel_camera::{
    PixelCameraPlugin, PixelZoom, PixelViewport
};
use bevy::window::WindowResized;
use bevy::window::WindowMode;
// Very good ressources overall : https://bevy-cheatbook.github.io/introduction.html

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: 320,
            height: 180,
        },
        PixelViewport,
    ));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(5.0),
            ..default()
        },
        texture: asset_server.load("test.png"),

        ..default()
    });
}

