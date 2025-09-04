mod strukture;
mod generator_polja;
mod display;
mod gameplay;


use crate::strukture::*;
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
struct Tezavnost {
    velikost: (usize,usize),
    st_min: usize,
}

pub const EAZY : Tezavnost = Tezavnost {
    velikost : (8,8),
    st_min : 12,
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

    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
    pub fn splash_plugin(app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

     // Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

 // Newtype to use a `Timer` for this screen as a resource

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("mina.png");
        // Display the logo
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
                    // This will set the logo to be 200px wide, and auto adjust its height
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
    pozicija: (usize,usize),
    global_pozicija: Vec2,
    is_flaged: bool,
    is_odprto: bool,
}

mod game {
    use bevy::{
        color::palettes::basic::{BLUE, LIME}, math::ops::abs, prelude::*
    };
    
    use crate::{ handle_click, strukture::Mreza, LeftClick, RightClick};
    
    use super::{despawn_screen, GameState, TEXT_COLOR};
    
    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (game_setup, setup_clock))
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_click)
        .add_systems(Update, update_clock)
        .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGameScreen>, despawn_screen::<ClockDisplay>))
        .add_event::<LeftClick>()
        .add_event::<RightClick>()
        .add_observer(odpri_tile)
        .add_observer(flag_polje)
        
        ;
        
    }



fn odpri_tile (
    trigger: Trigger<LeftClick>,
    mut query : Query<(&mut Sprite, &mut BevyTile)>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite,mut tile) in &mut query  {
        let poz = trigger.event().poz;
        let tile_poz = tile.global_pozicija;
        if (tile.is_flaged == false) && abs(tile_poz.x - poz.x) < 17.5 && abs(tile_poz.y - poz.y) < 17.5   {
            sprite.image = asset_server.load(tile.uncovered.clone());
            tile.is_odprto = true;
        }
    }
}

fn flag_polje (
    trigger: Trigger<RightClick>,
    mut query : Query<(&mut Sprite, &mut BevyTile)>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite,mut tile) in &mut query  {
        let poz = trigger.event().poz;
        let tile_poz = tile.global_pozicija;
        if (tile.is_odprto == false) && abs(tile_poz.x - poz.x) < 17.5 && abs(tile_poz.y - poz.y) < 17.5   {

            if tile.is_flaged {
                sprite.image = asset_server.load(tile.covered.clone());
                tile.is_flaged = false;
            } else {
                sprite.image = asset_server.load(tile.flaged.clone());
                tile.is_flaged = true;
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


fn update_clock(timey: Res<Time>, query: Query<(&mut Text, &mut ClockDisplay)>) {
    
    for (mut text, mut clock) in query{
        let elapsed = clock.time.elapsed().as_secs_f32() - 1.0;
        clock.time.tick(Time::delta(&timey));
        let seconds = elapsed as u32;
        let time_str = if seconds < 10 {format!("00{seconds}")} else if seconds < 100 {format!("0{seconds}")} else {format!("{seconds}")};

        text.0 = time_str;
        
    }

}



#[derive(Component)]
struct OnGameScreen;

use crate::Tezavnost;
use crate::BevyTile;

fn setup_clock(mut commands: Commands, tezavnost: Res<Tezavnost>) {
    let timer = ((
        Text::new("000"),
        ClockDisplay{time: Stopwatch::new().tick(Duration::from_secs_f32(1.0)).clone()}
    )
    ,Transform::from_translation(vec3((tezavnost.velikost.0 as f32 + 1.0) * 35.0, 0. as f32,0.))
        );
    commands.spawn(timer);
}

    fn game_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        tezavnost: Res<Tezavnost>,
        time: Res<Time>
    ) {

        let mut mreza = Mreza::safe_new(tezavnost.velikost, tezavnost.st_min, time.elapsed().as_millis() as u64);
        
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
                    pozicija : (i,j),
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
) {
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
            // println!("LeftClick");
        }
        if mouse_button_input.just_pressed(MouseButton::Right) {
            commands.trigger(RightClick {poz:pos});
            //  println!("RightClick");
        }
        }

    }

mod menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::CRIMSON,
        ecs::spawn::{SpawnIter, SpawnWith},
        prelude::*,
    };

use crate::{BevyTile, EAZY, HARD, INSANE, MEDIUM};

use super::{despawn_screen, GameState , TEXT_COLOR, Tile};

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
        Custom,
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
        Custom,
        BackToMainMenu,
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

fn main_menu_setup(mut commands: Commands) {
    let button_node = Node {
            width: Val::Px(300.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };  
        let button_icon_node = Node {
            width: Val::Px(30.0),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            ..default() 
        };
        let button_text_font = TextFont {
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
                        Text::new("Cistilec min"),
                        TextFont {
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
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Custom,
                        children![  
                            (
                                Text::new("Custom"),
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
                    MenuButtonAction::Custom => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::BackToMainMenu => {
                        game_state.set(GameState::Menu);
                        menu_state.set(MenuState::Main);
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
