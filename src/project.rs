pub struct Project {
    name: String,
    target_amount: u64,
    min_contribution: u64,
    pot_balance: u64,
}

impl Project {
    pub fn new(name: String, target_amount: u64, min_contribution: u64) -> Self {
        Project {
            name,
            target_amount,
            min_contribution,
            pot_balance: 0,
        }
    }

    pub fn contribute(&mut self, user_id: u64, amount: u64, blockchain: &mut Blockchain) {
        if amount < self.min_contribution {
            println!("Contribution too small.");
            return;
        }

        self.pot_balance += amount;
        blockchain.record_contribution(user_id, amount);
    }

    pub fn check_target_reached(&self) -> bool {
        self.pot_balance >= self.target_amount
    }

    pub fn disburse_funds(&self, blockchain: &mut Blockchain) {
        if self.pot_balance >= self.target_amount {
            // Disburse funds to the project owner (not implemented here)
            println!("Funds disbursed to project owner.");
            blockchain.record_disbursement(self.pot_balance);
        }
    }
}
