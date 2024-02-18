fn main() {

    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let mut missiles =8;
    let ready = 2;

    missiles = missiles - ready;
    println!("Firing {} of my {} misssiles", ready, missiles);
}
