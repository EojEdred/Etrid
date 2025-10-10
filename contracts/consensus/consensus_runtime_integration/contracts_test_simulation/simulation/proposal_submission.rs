pub fn submit_proposal(peer_id: &str, proposal: &str) -> bool {
    println!("{} submitted proposal: {}", peer_id, proposal);
    !proposal.is_empty()
}