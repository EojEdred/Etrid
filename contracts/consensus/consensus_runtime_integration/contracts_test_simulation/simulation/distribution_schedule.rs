pub fn distribute_rewards(peer_id: &str, reward: f64) -> bool {
    println!("Distributed {} tokens to {}", reward, peer_id);
    reward > 0.0
}