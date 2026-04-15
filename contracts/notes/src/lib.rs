#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, Address, Map};

#[contracttype]
#[derive(Clone)]
pub struct Content {
    pub author: Address,
    pub secret_payload: String, // Data rahasia yang dijual
    pub price: i128,            // Harga untuk membuka data ini
    pub title: String,
}

#[contract]
pub struct SorobanGatekeeper;

const CONTENTS: Symbol = symbol_short!("CONT");
const ACCESS_LIST: Symbol = symbol_short!("ACCESS"); // Siapa yang sudah beli apa
const BALANCE: Symbol = symbol_short!("BAL");      // Saldo kreator

#[contractimpl]
impl SorobanGatekeeper {
    
    // 1. Kreator memposting konten berbayar
    pub fn post_content(env: Env, author: Address, title: String, payload: String, price: i128) {
        author.require_auth();
        let mut all_content: Map<u64, Content> = env.storage().instance().get(&CONTENTS).unwrap_or(Map::new(&env));
        
        let content_id = all_content.len() as u64;
        let new_content = Content {
            author,
            secret_payload: payload,
            price,
            title,
        };
        
        all_content.set(content_id, new_content);
        env.storage().instance().set(&CONTENTS, &all_content);
    }

    // 2. User membayar untuk mendapatkan akses (Simulasi pembayaran)
    pub fn purchase_access(env: Env, buyer: Address, content_id: u64) {
        buyer.require_auth();
        
        let all_content: Map<u64, Content> = env.storage().instance().get(&CONTENTS).expect("No content found");
        let content = all_content.get(content_id).expect("Content ID invalid");

        // Logika akses: Simpan pasangan (buyer, content_id) ke storage
        let mut access: Map<Address, Vec<u64>> = env.storage().instance().get(&ACCESS_LIST).unwrap_or(Map::new(&env));
        let mut user_purchases = access.get(buyer.clone()).unwrap_or(Vec::new(&env));
        
        // Cek jika sudah pernah beli
        for id in user_purchases.iter() {
            if id == content_id { panic!("Already purchased"); }
        }

        user_purchases.push_back(content_id);
        access.set(buyer, user_purchases);
        env.storage().instance().set(&ACCESS_LIST, &access);

        // Update saldo kreator (Simulasi internal)
        let mut balances: Map<Address, i128> = env.storage().instance().get(&BALANCE).unwrap_or(Map::new(&env));
        let current_bal = balances.get(content.author.clone()).unwrap_or(0);
        balances.set(content.author, current_bal + content.price);
        env.storage().instance().set(&BALANCE, &balances);
    }

    // 3. Hanya pembeli yang bisa melihat isi konten
    pub fn unlock_content(env: Env, viewer: Address, content_id: u64) -> String {
        viewer.require_auth();
        
        let access: Map<Address, Vec<u64>> = env.storage().instance().get(&ACCESS_LIST).unwrap_or(Map::new(&env));
        let user_purchases = access.get(viewer.clone()).expect("You haven't bought any content");

        let mut has_access = false;
        for id in user_purchases.iter() {
            if id == content_id { has_access = true; break; }
        }

        if !has_access {
            panic!("Access denied: Please purchase first");
        }

        let all_content: Map<u64, Content> = env.storage().instance().get(&CONTENTS).unwrap_or(Map::new(&env));
        all_content.get(content_id).unwrap().secret_payload
    }
}
