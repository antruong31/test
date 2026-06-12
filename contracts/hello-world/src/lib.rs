#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Vec, symbol_short
};

#[contract]
pub struct TranslationContract;

#[contracttype]
#[derive(Clone)]
pub struct TranslationRecord {
    pub user: Address,
    pub source_lang: String,
    pub target_lang: String,
    pub original_text: String,
    pub translated_text: String,
}

const RECORDS: soroban_sdk::Symbol = symbol_short!("RECORDS");

#[contractimpl]
impl TranslationContract {

    pub fn save_translation(
        env: Env,
        user: Address,
        source_lang: String,
        target_lang: String,
        original_text: String,
        translated_text: String,
    ) {
        user.require_auth();

        let mut records: Vec<TranslationRecord> =
            env.storage()
                .persistent()
                .get(&RECORDS)
                .unwrap_or(Vec::new(&env));

        let record = TranslationRecord {
            user,
            source_lang,
            target_lang,
            original_text,
            translated_text,
        };

        records.push_back(record);

        env.storage()
            .persistent()
            .set(&RECORDS, &records);
    }

    pub fn get_all(env: Env) -> Vec<TranslationRecord> {
        env.storage()
            .persistent()
            .get(&RECORDS)
            .unwrap_or(Vec::new(&env))
    }
}