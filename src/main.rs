mod strukture;
mod generator_polja;
mod display;

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Tezavnost {
    velikost: (usize,usize),
    st_min: usize,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct KonecIgre {bool : bool}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct SteviloNeodkritih {stevilo: usize}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct SteviloMin {stevilo: i32}


pub const EAZY : Tezavnost = Tezavnost {
    velikost : (8,8),
    st_min : 10,
};

pub const MEDIUM : Tezavnost = Tezavnost {
    velikost : (16,16),
    st_min : 35,
};

pub const HARD : Tezavnost = Tezavnost {
    velikost : (28,16),
    st_min : 80,
};

pub const INSANE : Tezavnost = Tezavnost {
    velikost : (28,20),
    st_min : 120,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Tezavnost{velikost: (8,8),st_min: 10})
        .insert_resource(SteviloNeodkritih{stevilo:0})
        .insert_resource(SteviloMin{stevilo:0})
        .insert_resource(KonecIgre {bool:false})
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}



mod splash {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    pub fn splash_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("mina.png");
        commands.spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            OnSplashScreen,
            children![(     
                ImageNode::new(icon),
                Node {
                    width: Val::Px(200.0),
                    ..default()
                },
            )],
        ));
         commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }
}



#[derive(Component)]
struct BevyTile {
    vsebina: strukture::Tile,
    covered: String,
    uncovered: String,
    flaged: String,
    global_pozicija: Vec2,
    is_flaged: bool,
    is_odprto: bool,
}

mod game {
    use bevy::{
        math::ops::abs, prelude::*
    };
    
    use crate::{handle_click, strukture::{Mreza, Vsebina}, KonecIgre, LeftClick, RightClick, SteviloMin, SteviloNeodkritih};
    
    use super::{despawn_screen, GameState};
    
    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (game_setup, setup_clock, setup_count))
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(Update, (handle_click, update_clock, update_count))
        .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGameScreen>, despawn_screen::<ClockDisplay>, despawn_screen::<Counter>, despawn_screen::<Text>))
        .add_event::<LeftClick>()
        .add_event::<RightClick>()
        .add_event::<GameOver>()
        .add_event::<GameWon>()
        .add_observer(odpri_tile)
        .add_observer(flag_polje)
        .add_observer(game_over)
        .add_observer(game_won)
        
        ;
        
    }


fn odpri_tile (
    trigger: Trigger<LeftClick>,
    mut query : Query<(&mut Sprite, &mut BevyTile)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut st_ostalih: ResMut<SteviloNeodkritih>,
) {
    for (mut sprite,mut tile) in &mut query  {
        let poz = trigger.event().poz;
        let tile_poz = tile.global_pozicija;
        if (tile.is_odprto == false) && (tile.is_flaged == false) && abs(tile_poz.x - poz.x) < 17.5 && abs(tile_poz.y - poz.y) < 17.5   {
            sprite.image = asset_server.load(tile.uncovered.clone());
            tile.is_odprto = true;
            
            if tile.vsebina.vsebina == Vsebina::Stevilo(0) {
                commands.trigger(LeftClick {poz:(poz + vec2(35. , 0. ))});
                commands.trigger(LeftClick {poz:(poz + vec2(35. , 35. ))});
                commands.trigger(LeftClick {poz:(poz + vec2(0. , 35. ))});
                commands.trigger(LeftClick {poz:(poz + vec2(-35. , 35. ))});
                commands.trigger(LeftClick {poz:(poz + vec2(-35. , 0.))});
                commands.trigger(LeftClick {poz:(poz + vec2(-35. , -35. ))});
                commands.trigger(LeftClick {poz:(poz + vec2(0. , -35. ))});
                commands.trigger(LeftClick {poz:(poz + vec2(35. , -35. ))});
                // Ne razumem zakaj, ampak to NE DELA POČASI
            }
            if tile.vsebina.vsebina == Vsebina::Mina {
                sprite.color = Color::srgba(1.,0.,0., 1.);
                commands.trigger(GameOver);
            } else {
                st_ostalih.stevilo -= 1;
                if st_ostalih.stevilo == 0 {
                    commands.trigger(GameWon);
                }
            }

        }
    }
}

#[derive(Event)]    
struct GameOver;

#[derive(Event)]    
struct GameWon;


fn game_over (
    _trigger: Trigger<GameOver>,
    mut query : Query<(&mut Sprite, &mut BevyTile)>,
    asset_server: Res<AssetServer>,
    mut konec: ResMut<KonecIgre>,
    mut commands: Commands
) {
        for (mut sprite,mut tile) in &mut query  {
            if (tile.is_odprto == false) && tile.vsebina.vsebina == Vsebina::Mina {
                tile.is_odprto = true;
                sprite.image = asset_server.load(tile.uncovered.clone());
            }
        }
        konec.bool = true;
        commands.spawn((Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            Text::new(" You lose! \n Press Esc"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
            font: asset_server.load("times.ttf"),
            font_size: 55.0,
            ..default()
            },
        )
        );
}

fn game_won (
    _trigger: Trigger<GameWon>,
    asset_server: Res<AssetServer>,
    mut konec: ResMut<KonecIgre>,
    mut query : Query<(&mut Sprite, &mut BevyTile)>,
    mut commands: Commands
) {
        for (mut sprite, mut tile) in &mut query  {
            if tile.vsebina.vsebina == Vsebina::Mina {
                tile.is_odprto = true;
                sprite.image = asset_server.load(tile.flaged.clone());
            }
        }
        konec.bool = true;
        commands.spawn((Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            Text::new(" You win! \n Press Esc"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextFont {
            font: asset_server.load("times.ttf"),
            font_size: 55.0,
            ..default()
            },
        )
    );
}



fn flag_polje (
    trigger: Trigger<RightClick>,
    mut query : Query<(&mut Sprite, &mut BevyTile)>,
    asset_server: Res<AssetServer>,
    mut st_min: ResMut<SteviloMin>,
) {
    for (mut sprite,mut tile) in &mut query  {
        let poz = trigger.event().poz;
        let tile_poz = tile.global_pozicija;
        if (tile.is_odprto == false) && abs(tile_poz.x - poz.x) < 17.5 && abs(tile_poz.y - poz.y) < 17.5   {

            if tile.is_flaged {
                sprite.image = asset_server.load(tile.covered.clone());
                tile.is_flaged = false;
                st_min.stevilo += 1;
            } else {
                sprite.image = asset_server.load(tile.flaged.clone());
                tile.is_flaged = true;
                st_min.stevilo -= 1;
            }
        }
    }
}

use bevy::time::Stopwatch;
use std::time::Duration;

#[derive(Component)]
struct ClockDisplay{
    time: Stopwatch
}

#[derive(Component)]
struct Counter;

fn update_clock(timey: Res<Time>,konec:Res<KonecIgre> , query: Query<(&mut Text, &mut ClockDisplay)>) {
    if konec.bool == false {
        for (mut text, mut clock) in query{
            let elapsed = clock.time.elapsed().as_secs_f32() - 1.0;
            clock.time.tick(Time::delta(&timey));
            let seconds = elapsed as u32;
            let time_str = if seconds < 10 {format!("00{seconds}")} else if seconds < 100 {format!("0{seconds}")} else {format!("{seconds}")};
            
            text.0 = time_str;
            
        }
    }

}

fn update_count(mine_res: Res<SteviloMin>, query: Query<(&mut Text, &mut Counter)>) {
    let st_min = mine_res.stevilo; 
    for (mut text, _) in query{
        text.0 = format!["Število min: {st_min}"]
    }
}

#[derive(Component)]
struct OnGameScreen;

use crate::Tezavnost;
use crate::BevyTile;


fn setup_count(mut commands: Commands, mine_res: Res<SteviloMin>, asset_server: Res<AssetServer>) {
    let st_min = mine_res.stevilo; 
    let counter = (
        Node {position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        ..default()},
        Text::new(format!["st_min: {st_min}"]),
        TextFont {
            font: asset_server.load("times.ttf"),
            font_size: 55.0,
            ..default()
        },
        Counter);
    commands.spawn(counter);
}

fn setup_clock(mut commands: Commands,asset_server: Res<AssetServer>) {
    let timer = (
        Node {position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        right: Val::Px(0.0),
        ..default()},
        Text::new("000"),
        TextFont {
            font: asset_server.load("times.ttf"),
            font_size: 55.0,
            ..default()
        },
        ClockDisplay{time: Stopwatch::new().tick(Duration::from_secs_f32(1.0)).clone()}
    );
    commands.spawn(timer);
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tezavnost: Res<Tezavnost>,
    time: Res<Time>,
    mut st_ostalih: ResMut<SteviloNeodkritih>,
    mut st_min: ResMut<SteviloMin>,
    mut konec: ResMut<KonecIgre>,
    ) {

        let mreza = Mreza::safe_new(tezavnost.velikost, tezavnost.st_min, time.elapsed().as_millis() as u64);
        st_ostalih.stevilo = tezavnost.velikost.0 * tezavnost.velikost.1 - tezavnost.st_min;
        st_min.stevilo = tezavnost.st_min as i32;
        konec.bool = false;

        for i in 0..mreza.velikost.0 {
        
        for j in 0..mreza.velikost.1 {
            let new_tile = Option::expect(mreza.tile((i,j)), "ERROR: narobe generirana mreža");
            let (covered_png,uncovered_png, flaged_png) = new_tile.png_selections();
            commands.spawn(
            (
                Sprite {
                    image: asset_server.load(covered_png.clone()),
                    custom_size: Some(Vec2::new(35., 35.)),
                    ..Default::default()
            },
            Transform::from_translation(vec3((i as f32 + 0.5) * 35. - (mreza.velikost.0 as f32) / 2.0 * 35., (j as f32 + 0.5) * 35. - (mreza.velikost.1 as f32) / 2.0 * 35. , 0.)),
                BevyTile 
                {
                    vsebina : *new_tile,
                    covered : covered_png,
                    uncovered : uncovered_png,
                    flaged : flaged_png,
                    global_pozicija: (vec2((i as f32 + 0.5) * 35. - (mreza.velikost.0 as f32) / 2.0 * 35., (j as f32 + 0.5) * 35. - (mreza.velikost.1 as f32) / 2.0 * 35.)),
                    is_flaged : false,
                    is_odprto : false,
                },
         
            ));
            };
            
        }
    }

 fn game(
        keys: Res<ButtonInput<KeyCode>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        if keys.just_pressed(KeyCode::Escape) {
            game_state.set(GameState::Menu);
        }
     }
}

#[derive(Event)]
struct LeftClick {poz: Vec2}

#[derive(Event)]
struct RightClick {poz: Vec2}


fn handle_click (
     mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
    konec: Res<KonecIgre>,
) {
    if konec.bool {
        return;
    }
    let Ok(windows) = windows.single() else {
        return;
    };
    let (camera, camera_transform) = *camera;
    if let Some(pos) = windows
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.trigger(LeftClick {poz:pos});
        }
        if mouse_button_input.just_pressed(MouseButton::Right) {
            commands.trigger(RightClick {poz:pos});
        }
        }

    }

mod menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::CRIMSON,
        prelude::*,
    };

use crate::{BevyTile, EAZY, HARD, INSANE, MEDIUM};

use super::{despawn_screen, GameState , TEXT_COLOR};

pub fn menu_plugin(app: &mut App) {
        app
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(OnExit(GameState::Game), despawn_screen::<BevyTile>)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            );
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        #[default]
        Disabled,
    }

    #[derive(Component)]
    struct OnMainMenuScreen;

    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    #[derive(Component)]
    struct SelectedOption;

#[derive(Component)]
    enum MenuButtonAction {
        Eazy,
        Medium,
        Hard,
        Insane,
        Quit,
    }

fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ){
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }


fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };  
        let button_text_font = TextFont {
            font: asset_server.load("times.ttf"),
            font_size: 33.0,
            ..default()
        };        

        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    (
                        Text::new("Čistilec min"),
                        TextFont {
                            font: asset_server.load("times.ttf"),
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Eazy,
                        children![
                            (
                                Text::new("Easy"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Medium,
                        children![
                            (
                                Text::new("Medium"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Hard,
                        children![
                            (
                                Text::new("Hard"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Insane,
                        children![
                            (
                                Text::new("Insane"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                     (
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Quit,
                        children![
                            (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}

use crate::Tezavnost;

fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
        mut tezavnost: ResMut<Tezavnost>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_events.write(AppExit::Success);
                    }
                    MenuButtonAction::Eazy => {
                        game_state.set(GameState::Game);
                        *tezavnost = EAZY;
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Medium => {
                        game_state.set(GameState::Game);
                        *tezavnost = MEDIUM;
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Hard => {
                        game_state.set(GameState::Game);
                        *tezavnost = HARD;
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Insane => {
                        game_state.set(GameState::Game);
                        *tezavnost = INSANE;
                        menu_state.set(MenuState::Disabled);
                    }
                }
            }
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
