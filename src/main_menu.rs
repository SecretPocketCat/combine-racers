use bevy::{audio::AudioSink, prelude::*};

use crate::{
    settings::{KeyboardLayout, KeyboardSetting, MusicSetting, SfxSetting},
    AudioAssets, GameAssets, GameState, MusicController,
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(buttons)
                    .with_system(play_button)
                    .with_system(keyboard_setting_button)
                    .with_system(music_setting_button)
                    .with_system(sfx_setting_button)
                    .with_system(sfx_volume)
                    .with_system(music_volume),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup_menu));
    }
}

#[derive(Component)]
struct MainMenuMarker;

#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct KeyboardSettingButton;

#[derive(Component)]
struct KeyboardSettingButtonText;
#[derive(Component)]
struct MusicSettingButton;
#[derive(Component)]
struct MusicSettingButtonText;
#[derive(Component)]
struct SfxSettingButton;

#[derive(Component)]
struct SfxSettingButtonText;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn setup_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    sfx: Res<SfxSetting>,
    music: Res<MusicSetting>,
    keyboard: Res<KeyboardSetting>,
) {
    info!("setup_menu");

    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(45.0)),
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 30.0,
        color: TEXT_COLOR,
    };
    let title_text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 60.0,
        color: TEXT_COLOR,
    };
    let subtitle_text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    let container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .insert(MainMenuMarker)
        .id();

    let title = commands
        .spawn_bundle(
            TextBundle::from_section("Combine-Racers", title_text_style).with_style(Style {
                margin: UiRect {
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .id();

    let play_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section("Play", button_text_style.clone()));
        })
        .insert(PlayButton)
        .id();

    let keyboard_settings_title = commands
        .spawn_bundle(
            TextBundle::from_section("Keyboard", subtitle_text_style.clone()).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
        )
        .id();

    let qwerty_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                    format!("{}", **keyboard),
                    button_text_style.clone(),
                ))
                .insert(KeyboardSettingButtonText);
        })
        .insert(KeyboardSettingButton)
        .id();

    let audio_settings_title = commands
        .spawn_bundle(
            TextBundle::from_section("Audio", subtitle_text_style).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
        )
        .id();

    let sfx_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                    format!("SFX {}%", **sfx),
                    button_text_style.clone(),
                ))
                .insert(SfxSettingButtonText);
        })
        .insert(SfxSettingButton)
        .id();

    let music_button = commands
        .spawn_bundle(ButtonBundle {
            style: button_style,
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                    format!("Music {}%", **music),
                    button_text_style,
                ))
                .insert(MusicSettingButtonText);
        })
        .insert(MusicSettingButton)
        .id();

    commands.entity(container).push_children(&[
        title,
        play_button,
        keyboard_settings_title,
        qwerty_button,
        audio_settings_title,
        sfx_button,
        music_button,
    ]);
}

fn buttons(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn play_button(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<PlayButton>)>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Clicked {
            state.set(GameState::Playing).unwrap();
        }
    }
}

fn keyboard_setting_button(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<KeyboardSettingButton>,
        ),
    >,
    mut setting: ResMut<KeyboardSetting>,
    mut text_query: Query<&mut Text, With<KeyboardSettingButtonText>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Clicked {
            **setting = match **setting {
                KeyboardLayout::Azerty => KeyboardLayout::Qwerty,
                KeyboardLayout::Qwerty => KeyboardLayout::Azerty,
            };

            for mut text in text_query.iter_mut() {
                text.sections[0].value = format!("{}", **setting);
            }
        }
    }
}

fn music_setting_button(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<MusicSettingButton>),
    >,
    mut setting: ResMut<MusicSetting>,
    mut text_query: Query<&mut Text, With<MusicSettingButtonText>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Clicked {
            if **setting == 0 {
                **setting = 100;
            } else {
                **setting -= 10;
            }

            for mut text in text_query.iter_mut() {
                text.sections[0].value = format!("Music {}%", **setting);
            }
        }
    }
}

fn sfx_setting_button(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<SfxSettingButton>),
    >,
    mut setting: ResMut<SfxSetting>,
    mut text_query: Query<&mut Text, With<SfxSettingButtonText>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Clicked {
            if **setting == 0 {
                **setting = 100;
            } else {
                **setting -= 10;
            }

            for mut text in text_query.iter_mut() {
                text.sections[0].value = format!("SFX {}%", **setting);
            }
        }
    }
}

fn sfx_volume(sfx_setting: Res<SfxSetting>, audio: Res<Audio>, game_audio: Res<AudioAssets>) {
    // Do not run when SfxSetting is first added by SavePlugin
    if !sfx_setting.is_changed() || sfx_setting.is_added() {
        return;
    }

    audio.play_with_settings(
        game_audio.trick.clone(),
        PlaybackSettings::ONCE.with_volume(**sfx_setting as f32 / 100.),
    );
}

fn music_volume(
    music_setting: Res<MusicSetting>,
    audio_sinks: Res<Assets<AudioSink>>,
    controller: Option<Res<MusicController>>,
) {
    // Do not run when MusicSetting is first added by SavePlugin
    if !music_setting.is_changed() || music_setting.is_added() {
        return;
    }

    if let Some(controller) = controller {
        if let Some(sink) = audio_sinks.get(&controller.0) {
            sink.set_volume(**music_setting as f32 / 100.)
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
