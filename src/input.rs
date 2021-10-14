use ggez::input::gamepad::gilrs::Button;
use ggez::input::keyboard::KeyCode;
use glam::Vec2;
use maplit::hashmap;
use std::{collections::HashMap, collections::HashSet, hash::Hash};

#[derive(Clone, Debug)]
pub struct InputData<C>
where C: Clone + Copy + Eq + Hash {
    buttons: HashSet<C>,
    left_axis: Vec2
}

impl <C> InputData<C> 
where C: Clone + Copy + Eq + Hash {
    pub fn new() -> Self {
        Self {
            buttons: HashSet::new(),
            left_axis: Vec2::zero()
        }
    }

    pub fn activate(&mut self, command: C) {
        self.buttons.insert(command);
    }

    pub fn deactivate(&mut self, command: C) {
        self.buttons.remove(&command);
    }

    pub fn contains(&self, command: C) -> bool {
        self.buttons.contains(&command) 
    }

    pub fn get_stick(&self) -> Vec2 {
        self.left_axis
    }
}


pub struct InputState<I, C>
where
    I: Eq + Hash,
    C: Clone + Copy + Eq +  Hash,
{
    input_cfg: HashMap<I, C>,
    current: InputData<C>,
    previous: InputData<C>
}

impl<I, C> InputState<I, C>
where
    I: Copy + Clone + Eq + Hash,
    C: Copy + Clone + Eq + Hash,
{
    pub fn new(input_cfg: HashMap<I, C>) -> Self {
        Self {
            input_cfg,
            current: InputData::new(),
            previous: InputData::new(),
        }
    }

    pub fn save_current(&mut self) {
        self.previous = self.current.clone();
    }

    pub fn activate(&mut self, player_input: I) {
        self.input_cfg
            .get(&player_input)
            .cloned()
            .map(|c| self.current.activate(c));
    }

    pub fn deactivate(&mut self, player_input: I) {
        self.input_cfg
            .get(&player_input)
            .cloned()
            .map(|c| self.current.deactivate(c));
    }

    pub fn activate_command(&mut self, player_command: C) {
        self.current.activate(player_command);
    }

    pub fn deactivate_command(&mut self, player_command: C) {
        self.current.deactivate(player_command);
    }

    pub fn active(&self, player_command: C) -> bool {
        self.current.contains(player_command)
    }

    pub fn inactive(&self, player_command: C) -> bool {
        !self.current.contains(player_command)
    }

    pub fn just_active(&self, player_command: C) -> bool {
        self.current.contains(player_command) && !self.previous.contains(player_command)
    }

    pub fn just_inactive(&self, player_command: C) -> bool {
        self.previous.contains(player_command) && !self.current.contains(player_command)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerInput {
    Key(ggez::input::keyboard::KeyCode),
    Button(ggez::input::gamepad::gilrs::Button),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerCommand {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotUp,
    RotDown,
    Start,
    Quit,
}

pub struct InputCfg {
    pub bindings: HashMap<PlayerInput, PlayerCommand>,
}

pub fn default_input_cfg() -> HashMap<PlayerInput, PlayerCommand> {
    hashmap! {
        PlayerInput::Key(KeyCode::Left) => PlayerCommand::MoveLeft,
        PlayerInput::Key(KeyCode::Right) => PlayerCommand::MoveRight,
        PlayerInput::Key(KeyCode::Down) => PlayerCommand::MoveDown,
        PlayerInput::Key(KeyCode::Z) => PlayerCommand::RotUp,
        PlayerInput::Key(KeyCode::X) => PlayerCommand::RotDown,
        PlayerInput::Key(KeyCode::Space) => PlayerCommand::Start,
        PlayerInput::Button(Button::DPadLeft) => PlayerCommand::MoveLeft,
        PlayerInput::Button(Button::DPadRight) => PlayerCommand::MoveRight,
        PlayerInput::Button(Button::DPadDown) => PlayerCommand::MoveDown,
        PlayerInput::Button(Button::LeftTrigger) => PlayerCommand::RotDown,
        PlayerInput::Button(Button::RightTrigger) => PlayerCommand::RotUp,
        PlayerInput::Button(Button::West) => PlayerCommand::RotUp,
        PlayerInput::Button(Button::South) => PlayerCommand::RotDown,
        PlayerInput::Button(Button::Start) => PlayerCommand::Start,
        PlayerInput::Button(Button::Select) => PlayerCommand::Quit,
    }
}
