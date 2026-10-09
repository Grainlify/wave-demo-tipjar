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

#[test]
fn progress_toward_goal() {
    let (env, c) = setup();
    let owner = Address::generate(&env);
    let a = Address::generate(&env);
    c.init(&owner);
    assert_eq!(c.progress(), 0);
    c.set_goal(&40);
    c.tip(&a, &10, &symbol_short!("go"));
    assert_eq!(c.progress(), 25);
    c.tip(&a, &100, &symbol_short!("big"));
    assert_eq!(c.progress(), 100);
}
