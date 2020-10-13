use bevy::prelude::*;
use bevy_ninepatch::*;

type NP = NinePatchBuilder<UiContentZone>;

#[derive(Debug)]
pub struct UiConfig {
    font: Handle<Font>,
    tex_panel: Handle<Texture>,
    np_panel: Handle<NP>,
    tex_button: Handle<Texture>,
    np_button: Handle<NP>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UiContentZone {
    PanelTitle,
    PanelInner,
    ButtonInner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiPanelId {
    SrvConnError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiButtonId {
    QuitGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiId {
    Panel(UiPanelId),
    Button(UiButtonId),
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NP>>,
) {
    let font = asset_server.load("assets/font.ttf").unwrap();

    let tex_panel = asset_server.load("assets/ui.png").unwrap();
    let tex_button = asset_server.load("assets/btn.png").unwrap();

    let patches_panel = vec![
        vec![
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(1.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::StretchRatio(1.0),
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(2.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::StretchRatio(0.0),
                y_growth: GrowthMode::None,
                content: Some(UiContentZone::PanelTitle),
            },
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(1.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::StretchRatio(1.0),
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
        ],
        vec![
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(1.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::StretchRatio(1.0),
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(36.0),
                height: PatchSize::Absolute(1.0),
                x_growth: GrowthMode::StretchRatio(1.0),
                y_growth: GrowthMode::StretchRatio(1.0),
                content: Some(UiContentZone::PanelInner),
            },
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(1.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::StretchRatio(1.0),
                content: None,
            },
        ],
        vec![
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(36.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::StretchRatio(1.0),
                y_growth: GrowthMode::None,
                content: None,
            },
            Patch {
                width: PatchSize::Absolute(16.0),
                height: PatchSize::Absolute(16.0),
                x_growth: GrowthMode::None,
                y_growth: GrowthMode::None,
                content: None,
            },
        ],
    ];

    let np_panel = nine_patches.add(NinePatchBuilder::from_patches(patches_panel));
    let np_button = nine_patches.add(
        NinePatchBuilder::by_margins(10.0, 10.0, 10.0, 10.0, UiContentZone::ButtonInner)
    );

    let uicfg = UiConfig {
        font, tex_panel, np_panel, tex_button, np_button,
    };

    spawn_ui(&mut commands, &uicfg, UiId::Panel(UiPanelId::SrvConnError));
    commands.insert_resource(uicfg);

    commands.spawn(UiCameraComponents::default());
}

pub fn spawn_ui(commands: &mut Commands, cfg: &UiConfig, ui: UiId) -> Entity {
    match ui {
        UiId::Panel(_) => {
            commands.spawn(NinePatchComponents {
                nine_patch_data: NinePatchData {
                    nine_patch: cfg.np_panel,
                    texture: cfg.tex_panel,
                    ..Default::default()
                },
                nine_patch_size: NinePatchSize(Vec2::new(256.0, 128.0)),
                ..Default::default()
            }).with(ui).current_entity().unwrap()
        }
        UiId::Button(_) => {
            commands.spawn(NinePatchComponents {
                nine_patch_data: NinePatchData {
                    nine_patch: cfg.np_button,
                    texture: cfg.tex_button,
                    ..Default::default()
                },
                ..Default::default()
            }).with(ui).current_entity().unwrap()
        }
    }
}

fn ui_content_provider(
    mut commands: Commands,
    cfg: Res<UiConfig>,
    q_parent: Query<&UiId>,
    mut q_elem: Query<(Entity, &mut NinePatchContent<UiContentZone>)>,
) {
    for (e_child, mut content) in &mut q_elem.iter() {
        if content.loaded {
            continue;
        }

        let e_parent = content.parent;
        let ui_id = q_parent.get::<UiId>(e_parent).unwrap();

        let e_content = match (*ui_id, content.content) {
            (UiId::Panel(UiPanelId::SrvConnError), UiContentZone::PanelTitle) => {
                commands.spawn(TextComponents {
                    text: Text {
                        value: "Connection Error".to_string(),
                        font: cfg.font,
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                }).current_entity().unwrap()
            },
            (UiId::Panel(UiPanelId::SrvConnError), UiContentZone::PanelInner) => {
                commands.spawn(TextComponents {
                    text: Text {
                        value: "Failed to connect to server.".to_string(),
                        font: cfg.font,
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                }).current_entity().unwrap()
            },
            (UiId::Button(UiButtonId::QuitGame), UiContentZone::ButtonInner) => {
                commands.spawn(TextComponents {
                    text: Text {
                        value: "Quit Game".to_string(),
                        font: cfg.font,
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                }).current_entity().unwrap()
            },
            _ => {
                panic!("unknown UI zone");
            }
        };

        commands.push_children(e_child, &[e_content]);
        content.loaded = true;
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(NinePatchPlugin::<UiContentZone>::default())
        .add_startup_system(setup.system())
        .add_system(ui_content_provider.system())
        .run();
}
