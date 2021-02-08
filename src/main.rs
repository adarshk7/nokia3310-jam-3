mod electron;
mod plate;

use bevy::prelude::*;
use plate::PlateSelectedAnimationTimer;

use crate::electron::{electron_physics_system, Electron};
use crate::plate::{
    plate_control_system, polarity_indicator_board_system, Plate, PlateState,
    PolarityIndicatorBoard,
};

const WINDOW_HEIGHT: f32 = 48.0;
const WINDOW_WIDTH: f32 = 84.0;
const OUTER_WALL_THICKNESS: f32 = 1.0;
const ELECTRON_SIZE: f32 = 3.0;

const COLOR_LIGHT: &str = "c7f0d8";
const COLOR_DARK: &str = "43523d";

const FONT_SIZE: f32 = 4.0;
const TEXT_POSITION_TOP: f32 = 1.0;
const TEXT_POSITION_LEFT: f32 = 2.0;

const PLATE_ANIMATION_TIMER_PERIOD: f32 = 0.25;

fn main() {
    let mut app = App::build();
    app.insert_resource(WindowDescriptor {
        title: "Nokia 3310 Game Jam 3".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        vsync: true,
        scale_factor_override: Some(10.0),
        ..Default::default()
    })
    .insert_resource(PolarityIndicatorBoard {
        polarity: PlateState::Negative,
    })
    .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.insert_resource(ClearColor(Color::hex(COLOR_LIGHT).unwrap()))
        .add_startup_system(setup.system())
        .add_system(electron_physics_system.system())
        .add_system(plate_control_system.system())
        .add_system(polarity_indicator_board_system.system())
        .insert_resource(PlateSelectedAnimationTimer {
            timer: Timer::from_seconds(PLATE_ANIMATION_TIMER_PERIOD, true),
        })
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
) {
    let material_foreground = materials.add(Color::hex(COLOR_DARK).unwrap().into());
    let texture_plate_off = materials.add(asset_server.load("textures/plate_off.png").into());

    let music = asset_server.load("sound/bad_melody.wav");
    audio.play(music);

    commands
        .spawn(UiCameraBundle::default())
        .spawn(OrthographicCameraBundle::new_2d())
        // Electron
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(ELECTRON_SIZE, ELECTRON_SIZE)),
            ..Default::default()
        })
        .with(Electron::new(10.0))
        // Outer wall
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(-(WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(OUTER_WALL_THICKNESS, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz((WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(OUTER_WALL_THICKNESS, WINDOW_HEIGHT)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material_foreground.clone(),
            transform: Transform::from_xyz(0.0, (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, OUTER_WALL_THICKNESS)),
            ..Default::default()
        })
        // Electric plates
        .spawn(SpriteBundle {
            material: texture_plate_off.clone(),
            transform: Transform::from_xyz(
                (WINDOW_WIDTH - OUTER_WALL_THICKNESS) / 2.0 - 1.0,
                0.0,
                0.0,
            ),
            ..Default::default()
        })
        .with(Plate::new(1))
        .spawn(SpriteBundle {
            material: texture_plate_off,
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    (WINDOW_HEIGHT - OUTER_WALL_THICKNESS) / 2.0 - 1.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                ..Default::default()
            },

            ..Default::default()
        })
        .with(Plate::new(2))
        .spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "POLARITY ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("font/EffortsPro.ttf"),
                            font_size: FONT_SIZE,
                            color: Color::hex(COLOR_DARK).unwrap(),
                        },
                    },
                    TextSection {
                        value: "OFF".to_string(),
                        style: TextStyle {
                            font: asset_server.load("font/EffortsPro.ttf"),
                            font_size: FONT_SIZE,
                            color: Color::hex(COLOR_DARK).unwrap(),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(TEXT_POSITION_TOP),
                    left: Val::Px(TEXT_POSITION_LEFT),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
}
