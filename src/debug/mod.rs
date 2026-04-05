use crate::debug::map::*;
use crate::state::GameState;
use bevy::prelude::*;

pub mod map;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugOn>()
            .init_resource::<DebugCommand>()
            .add_systems(Update, read_debug_keys.run_if(in_state(GameState::Playing)))
            .add_systems(Update, read_debug_commands.run_if(in_state(DebugOn::On)));
    }
}

#[derive(Resource, States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugOn {
    On,
    #[default]
    Off,
}

pub fn read_debug_keys(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<DebugOn>>,
    mut next_state: ResMut<NextState<DebugOn>>,
) {
    if input.just_pressed(KeyCode::F3) {
        match current_state.get() {
            DebugOn::Off => {
                next_state.set(DebugOn::On);
                info!("Debug mode: ON");
            }
            DebugOn::On => {
                next_state.set(DebugOn::Off);
                info!("Debug mode: OFF");
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct DebugCommand {
    pub current_text: String,
}

pub fn read_debug_commands(
    mut commands: Commands,
    mut debug_cmd: ResMut<DebugCommand>,
    mut char_evr: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<crate::characters::input::Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 1.Backspace
    if keys.just_pressed(KeyCode::Backspace) {
        debug_cmd.current_text.pop();
    }

    // 2.Typing
    for ev in char_evr.read() {
        if !ev.state.is_pressed() {
            continue;
        }

        if let bevy::input::keyboard::Key::Character(c) = &ev.logical_key {
            debug_cmd.current_text.push_str(c.as_str());
        }
    }

    // 3.Enter
    if keys.just_pressed(KeyCode::Enter) {
        let full_command = debug_cmd.current_text.clone();
        debug_cmd.current_text.clear();

        // Split by spaces
        let parts: Vec<&str> = full_command.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        // --- MATCH ARM 1 ---
        match parts[0] {
            "/spawn" => {
                // --- MATCH ARM 2 ---
                let option = parts.get(1).unwrap_or(&"_");

                match *option {
                    "tilemap" => {
                        // --- MATCH ARM 3 ---
                        if let Some(target) = parts.get(2) {
                            debug_spawn_sprites(
                                &mut commands,
                                target,
                                &player_query,
                                &asset_server,
                                &mut texture_atlas_layouts,
                            );
                        } else {
                            warn!("Usage: /spawn tilemap <name>");
                        }
                    }
                    _ => warn!("Unknown spawn option: {}", option),
                }
            }
            "/help" => info!("Available: /spawn tilemap <name>"),
            _ => warn!("Unknown command: {}", parts[0]),
        }
    }
}
