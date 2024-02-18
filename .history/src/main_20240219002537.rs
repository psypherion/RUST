const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    const  READY_AMOUNT = 2;
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    missiles = missiles - ready;
    println!("Firing {} of my {} misssiles", ready, missiles);
}
