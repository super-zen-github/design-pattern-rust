/// State design patter [action]
/// three ways:
/// 1. if else
/// 2. query table
/// 3. state mode

pub struct MarioStateMachine {
    current_state: String,
    score: i64,
}

impl MarioStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: "".into(),
            score: 0,
        }
    }

    fn set_state(&mut self, state: String) {
        self.current_state = state;
    }

    pub fn get_state(&self) -> &str {
        &self.current_state
    }

    fn set_score(&mut self, score: i64) {
        self.score = score;
    }
    
    pub fn get_score(&self) -> i64 {
        self.score
    }
}

pub trait IMario {
    fn get_name(&self) -> String;
    fn obtain_mushroom(&self, machine: &mut MarioStateMachine);
    fn encounter_monster(&self, machine: &mut MarioStateMachine);
}

pub struct SmallMario {}

impl SmallMario {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_state() -> String {
        "SmallMario".into()
    }
}

impl IMario for SmallMario {
    fn get_name(&self) -> String {
        "SmallMario".into()
    }

    fn obtain_mushroom(&self, machine: &mut MarioStateMachine) {
        machine.set_state(SuperMario::get_state());
        machine.set_score(machine.get_score() + 100);
    }

    fn encounter_monster(&self, machine: &mut MarioStateMachine) {
        println!("Dead");
    }
}


pub struct SuperMario {}

impl SuperMario {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_state() -> String {
        "SuperMario".into()
    }
}

impl IMario for SuperMario {
    fn get_name(&self) -> String {
        "SuperMario".into()
    }

    fn obtain_mushroom(&self, machine: &mut MarioStateMachine) {
        machine.set_score(machine.get_score() + 100);
    }

    fn encounter_monster(&self, machine: &mut MarioStateMachine) {
        machine.set_state(SmallMario::get_state());
        machine.set_score(machine.get_score() - 100);
    }
}
