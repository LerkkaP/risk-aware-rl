use std::collections::HashMap;

mod rewards;
mod transitions;
mod mdp;

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
        rewards: rewards_original,
        transitions: transitions,
        actlist: actlist
    };

   let a = mdp.step(0, 2);

   println!("{}", a.0);

}
