mod rewards;
mod transitions;
mod mdp;

fn main() {
    let transitions = transitions::get_transitions();
    let rewards_original = rewards::get_rewards_original();

    let mdp = mdp::MDP {
        rewards: rewards_original,
        transitions: transitions
    };
}
