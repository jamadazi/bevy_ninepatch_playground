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
    textstyle: TextStyle,
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
    LobbyMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiButtonId {
    ReadyPlayer,
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
        //NinePatchBuilder::by_margins(5.0, 10.0, 6.0, 6.0, UiContentZone::ButtonInner)
    );

    let textstyle = TextStyle {
        font_size: 16.0,
        color: Color::WHITE,
    };

    let uicfg = UiConfig {
        font, tex_panel, np_panel, tex_button, np_button, textstyle
    };

    //spawn_ui(&mut commands, &uicfg, UiId::Button(UiButtonId::QuitGame));
    //spawn_ui(&mut commands, &uicfg, UiId::Panel(UiPanelId::SrvConnError));
    spawn_ui(&mut commands, &uicfg, UiId::Panel(UiPanelId::LobbyMenu));

    commands.insert_resource(uicfg);

    commands.spawn(UiCameraComponents::default());
}

pub fn spawn_ui(commands: &mut Commands, cfg: &UiConfig, ui: UiId) -> Entity {
    match ui {
        UiId::Panel(_) => {
            commands.spawn(NinePatchComponents {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(420.0), Val::Px(240.0)),
                    //this doesn't work
                    //size: Size::new(Val::Auto, Val::Auto),
                    ..Default::default()
                },
                nine_patch_data: NinePatchData {
                    nine_patch: cfg.np_panel,
                    texture: cfg.tex_panel,
                    ..Default::default()
                },
                ..Default::default()
            }).with(ui).current_entity().unwrap()
        }
        UiId::Button(_) => {
            commands.spawn(NinePatchComponents {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(128.0), Val::Px(48.0)),
                    //this doesn't work
                    //size: Size::new(Val::Auto, Val::Auto),
                    ..Default::default()
                },
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

fn spawn_text(commands: &mut Commands, cfg: &UiConfig, value: &str) -> Entity {
    commands.spawn(TextComponents {
        /*
        style: Style {
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        */
        text: Text {
            value: value.to_string(),
            font: cfg.font,
            style: cfg.textstyle.clone(),
        },
        ..Default::default()
    }).current_entity().unwrap()
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
                spawn_text(&mut commands, &cfg, "Connection Error")
            },
            (UiId::Panel(UiPanelId::SrvConnError), UiContentZone::PanelInner) => {
                spawn_text(&mut commands, &cfg, "Failed to connect to server.")
            },
            (UiId::Panel(UiPanelId::LobbyMenu), UiContentZone::PanelTitle) => {
                spawn_text(&mut commands, &cfg, "Game Lobby")
            },
            (UiId::Panel(UiPanelId::LobbyMenu), UiContentZone::PanelInner) => {
                let e = commands.spawn(NodeComponents {
                    style: Style {
                        margin: Rect::all(Val::Auto),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                }).current_entity().unwrap();

                let btn_ready = spawn_ui(&mut commands, &cfg, UiId::Button(UiButtonId::ReadyPlayer));
                let btn_quit = spawn_ui(&mut commands, &cfg, UiId::Button(UiButtonId::QuitGame));

                commands.push_children(e, &[
                    btn_quit,
                    btn_ready,
                ]);

                e
            },
            (UiId::Button(UiButtonId::ReadyPlayer), UiContentZone::ButtonInner) => {
                spawn_text(&mut commands, &cfg, "Ready to play!")
            },
            (UiId::Button(UiButtonId::QuitGame), UiContentZone::ButtonInner) => {
                spawn_text(&mut commands, &cfg, "Quit Game")
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
