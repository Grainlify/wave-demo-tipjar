use super::*;
use soroban_sdk::{testutils::Address as _, Env};

fn setup() -> (Env, TipJarClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(TipJar, ());
    let c = TipJarClient::new(&env, &id);
    (env, c)
}

#[test]
fn tips_add_up() {
    let (env, c) = setup();
    let owner = Address::generate(&env);
    let a = Address::generate(&env);
    c.init(&owner);
    assert_eq!(c.tip(&a, &5, &symbol_short!("thanks")), 5);
    assert_eq!(c.tip(&a, &7, &symbol_short!("more")), 12);
    assert_eq!(c.tipped_by(&a), 12);
}

#[test]
fn zero_tip_is_refused() {
    let (env, c) = setup();
    let owner = Address::generate(&env);
    c.init(&owner);
    assert_eq!(c.try_tip(&owner, &0, &symbol_short!("x")), Err(Ok(Error::ZeroTip)));
}
