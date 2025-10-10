use uuid::Uuid;

#[derive(Debug)]
pub struct Voter {
    pub id: String,
    pub staked_tokens: u64,
    pub eligible: bool,
}

impl Voter {
    pub fn new(staked_tokens: u64) -> Self {
        let id = Uuid::new_v4().to_string();
        Voter {
            id,
            staked_tokens,
            eligible: staked_tokens >= 64,
        }
    }
}
