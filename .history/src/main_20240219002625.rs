const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    missiles = missiles - ready;
    println!("Firing {} of my {} missiles", ready, missiles);
}
