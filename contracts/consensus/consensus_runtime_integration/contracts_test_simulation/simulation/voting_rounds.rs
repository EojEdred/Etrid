pub fn cast_vote(peer_id: &str, proposal_id: u32, weight: u64) -> bool {
    println!("{} voted on proposal {} with weight {}", peer_id, proposal_id, weight);
    weight > 0
}