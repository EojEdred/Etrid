pub fn process_stake(peer_id: &str, amount: u64) -> bool {
    println!("Staking {} coins for peer {}", amount, peer_id);
    amount >= 1
}