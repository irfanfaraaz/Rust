/// A simple savings account implementation

pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Deposit amount must be positive");
        }
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) -> Result<i32, String> {
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(amount)
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn transfer(&mut self, amount: i32, acc_number: u32) -> Result<String, String> {
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(format!("Transferred {} to account {}", amount, acc_number))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_initial_balance_of_zero() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_deposit_amount() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100, "Balance should be 100");
        assert_ne!(account.get_balance(), 0);
        assert!(account.get_balance() == 100);
    }

    #[test]
    #[should_panic]
    fn should_panic_on_negative_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(-100);
        // assert!(account.get_balance() >= 0);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        let result = account.transfer(50, 12345);
        assert_eq!(account.get_balance(), 50);
        assert_eq!(result, Ok("Transferred 50 to account 12345".to_string()));
        Ok(())
    }
}
