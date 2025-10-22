use bevy::prelude::*;
use crate::game::character_state::CharacterState;
use crate::direction::Direction8;
use crate::game::input::directional_input;

#[derive(Clone)]
pub struct CharacterInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action1: bool,
    pub action2: bool,
}

impl Default for CharacterInput {
    fn default() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            action1: false,
            action2: false,
        }
    }
}

impl CharacterInput {
    pub fn from_array(arr: [bool; 6]) -> Self {
        Self {
            up: arr[0],
            down: arr[1],
            left: arr[2],
            right: arr[3],
            action1: arr[4],
            action2: arr[5],
        }
    }
    
    pub fn as_array(&self) -> [bool; 6] {
        [
            self.up,
            self.down,
            self.left,
            self.right,
            self.action1,
            self.action2,
        ]
    }
}

#[derive(Component)]
pub struct RandomInput {
    pub input: CharacterInput,
}

impl Default for RandomInput {
    fn default() -> Self {
        Self {
            input: CharacterInput::default(),
        }
    }
}

// update random input system
pub fn update_random_input(
    mut query: Query<(&mut Direction8, &mut CharacterState, &mut RandomInput)>
) {
    let chance = 0.01; // chance per update to toggle an inputs state

    for (mut direction, mut state, mut random_input) in query.iter_mut() {
        
        let mut input_array = random_input.input.as_array();
        
        for i in 0..input_array.len() {
            if rand::random::<f32>() < chance {
                input_array[i] = !input_array[i];
            }
        }
    
        random_input.input = CharacterInput::from_array(input_array);

        let (new_direction, new_state) = directional_input([
            random_input.input.up,
            random_input.input.down,
            random_input.input.left,
            random_input.input.right,
        ]);

        if new_direction.is_some() {
            *direction = new_direction.unwrap();
        }
        *state = new_state;        
    }
}