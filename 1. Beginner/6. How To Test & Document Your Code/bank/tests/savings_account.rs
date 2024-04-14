use bank::SavingsAccount;

mod utils;

#[test]

fn should_have_initial_balance_of_zero() {
    utils::common_setup();
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);
}
