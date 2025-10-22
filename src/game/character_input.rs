use bevy::prelude::*;

#[derive(Component, Clone)]
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
    mut query: Query<(&mut RandomInput, &mut CharacterInput)>,
) {
    let chance = 0.004; // chance per update to toggle an inputs state

    for (mut random_input, mut character_input) in query.iter_mut() {
        
        let mut input_array = random_input.input.as_array();
        
        for i in 0..input_array.len() {
            if rand::random::<f32>() < chance {
                input_array[i] = !input_array[i];
            }
        }
    
        random_input.input = CharacterInput::from_array(input_array);

        *character_input = random_input.input.clone();     
    }
}