use soroban_sdk::{Address, Env, Symbol, panic_with_error};
use crate::auth::{assert_admin, AuthError};

pub fn write_admin(env: &Env, auth: &Address, admin: &Address) {
    auth.require_auth();
    let admin_key = Symbol::new(env, "admin");
    if env.storage().instance().has(&admin_key) {
        panic_with_error!(env, AuthError::AlreadyInitialized);
    }
    env.storage().instance().set(&admin_key, admin);
}

pub fn require_admin(env: &Env, auth: &Address) {
    let admin: Address = env.storage()
        .instance()
        .get(&Symbol::new(env, "admin"))
        .unwrap_or_else(|| panic_with_error!(env, AuthError::NotInitialized));
    assert_admin(env, auth, &admin);
}