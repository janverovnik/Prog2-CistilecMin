mod strukture;
mod generator_polja;
mod display;
mod gameplay;


use std::io;
// use bevy::{prelude::*};
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
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

mod game {
    use bevy::{
        color::palettes::basic::{BLUE, LIME},
        prelude::*,
    };

    use crate::strukture::Mreza;

    use super::{despawn_screen, GameState, TEXT_COLOR};

 pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }

#[derive(Component)]
    struct OnGameScreen;

  #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

fn game_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let mut mreza = Mreza::safe_new((8, 8), 12, 42);

        for j in 0..mreza.velikost.0 {
        
        for i in 0..mreza.velikost.1 {
            commands.spawn((Sprite {
            image: asset_server.load((Option::expect(mreza.tile((i, j)), "ERROR: narobe generirana mreža")).png_select()),
            custom_size: Some(Vec2::new(35., 35.)), // velikost 35. zgleda najbolj optimalna
            ..Default::default()
            },
            Transform::from_translation(vec3((j as f32) * 35.5 - (mreza.velikost.0 as f32) / 2.0 * 35.5, (i as f32) * 35.5 - (mreza.velikost.1 as f32) / 2.0 * 35.5, 0.)))); 
            // + 0.5 ker buffer
            }
            };
            
        }
    
    

    

fn game(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
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

use super::{despawn_screen, GameState , TEXT_COLOR};

pub fn menu_plugin(app: &mut App) {
        app
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
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

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        };
        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };        

        // let right_icon = asset_server.load(r"2.png");
        // let wrench_icon = asset_server.load(r"3.png");
        // let exit_icon = asset_server.load(r"4.png");

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
                    // Display the game name
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
                                Text::new("Eazy"),
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

fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_events.write(AppExit::Success);
                    }
                    MenuButtonAction::Eazy => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Medium => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Hard => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Insane => {
                        game_state.set(GameState::Game);
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