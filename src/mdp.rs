use std::collections::HashMap;

pub struct MDP {
    pub rewards: HashMap<(i32, i32, i32), i32>,
    pub transitions: HashMap<(i32, i32), HashMap<i32, f64>>
}

impl MDP {
    pub fn R(&self, state: &i32, action: &i32, next_state: &i32) -> &i32 {
        &self
        .rewards.get(&(*state, *action, *next_state))
        .expect("Reward not found.")

    } 

    pub fn T(&self, state: &i32, action: &i32) -> &HashMap<i32, f64> {
        &self
        .transitions.get(&(*state, *action))
        .expect("Transition not found.")
    }
}