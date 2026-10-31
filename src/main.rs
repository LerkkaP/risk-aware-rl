mod rewards;
mod transitions;

fn main() {
    let transitions = transitions::get_transitions();
    let rewards_original = rewards::get_rewards_original();
}
