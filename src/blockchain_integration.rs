pub struct Blockchain {
    contributions: Vec<(u64, u64)>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            contributions: Vec::new(),
        }
    }

    pub fn record_contribution(&mut self, user_id: u64, amount: u64) {
        self.contributions.push((user_id, amount));
    }

    pub fn record_disbursement(&mut self, amount: u64) {
        // Record disbursement in the simulated blockchain
    }
}
