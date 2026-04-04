use crate::debug::map::*;
use crate::state::GameState;
use bevy::prelude::*;

pub mod map;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugOn>()
            .add_systems(Update, read_debug_keys.run_if(in_state(GameState::Playing)))
            .add_systems(Update, debug_spawn_sprites.run_if(in_state(DebugOn::On)));
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
