use std::collections::HashMap;

mod rewards;
mod transitions;
mod mdp;
mod value_iteration;

fn main() {
    let transitions = transitions::get_transitions();
    let rewards_original = rewards::get_rewards_original();

    let actlist = HashMap::from([
        (0, vec![0, 1, 2]),
        (1, vec![0, 1]),
        (2, vec![0, 1]),
        (3, vec![0, 1]),
        (4, vec![]),
        (5, vec![]),
    ]);

    let terminal_states = [4, 5];

    let mdp = mdp::MDP {
        terminal_states: terminal_states,
        rewards: rewards_original.clone(),
        transitions: transitions,
        actlist: actlist
    };

    let optimal_value_function = value_iteration::optimal_value_function(&mdp);
    let optimal_policy = value_iteration::optimal_policy(&mdp, optimal_value_function);
}
