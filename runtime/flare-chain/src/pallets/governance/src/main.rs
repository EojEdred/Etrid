mod proposal;
mod voter;

use proposal::Proposal;
use voter::Voter;

fn main() {
    println!("Ëtrid Governance Engine live...");
    let mut proposal = Proposal::new("Elect New Director", "Vote to elect director 0xD34DB33F");
    proposal.cast_vote("peer1", true);
    proposal.cast_vote("peer2", false);
    println!("Proposal Summary: {:?}", proposal.summary());
}
