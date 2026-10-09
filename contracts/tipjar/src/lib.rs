#![no_std]
//! Tip jar: supporters record tips (points, no tokens) with a short note.
//! A demo project for Grainlify wave evidence on Stellar testnet.
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
enum Key {
    Owner,
    Total,
    By(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    ZeroTip = 3,
}

#[contract]
pub struct TipJar;

#[contractimpl]
impl TipJar {
    pub fn init(env: Env, owner: Address) -> Result<(), Error> {
        if env.storage().instance().has(&Key::Owner) {
            return Err(Error::AlreadyInitialized);
        }
        owner.require_auth();
        env.storage().instance().set(&Key::Owner, &owner);
        env.storage().instance().set(&Key::Total, &0u64);
        Ok(())
    }

    /// Records a tip of `amount` points from `from`. Returns the jar's new total.
    pub fn tip(env: Env, from: Address, amount: u32, note: Symbol) -> Result<u64, Error> {
        if !env.storage().instance().has(&Key::Owner) {
            return Err(Error::NotInitialized);
        }
        if amount == 0 {
            return Err(Error::ZeroTip);
        }
        from.require_auth();
        let total: u64 = env.storage().instance().get(&Key::Total).unwrap_or(0) + amount as u64;
        env.storage().instance().set(&Key::Total, &total);
        let mine: u64 = env.storage().persistent().get(&Key::By(from.clone())).unwrap_or(0) + amount as u64;
        env.storage().persistent().set(&Key::By(from.clone()), &mine);
        env.events().publish((symbol_short!("tip"), from), (amount, note));
        Ok(total)
    }

    pub fn total(env: Env) -> u64 {
        env.storage().instance().get(&Key::Total).unwrap_or(0)
    }

    pub fn tipped_by(env: Env, who: Address) -> u64 {
        env.storage().persistent().get(&Key::By(who)).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
