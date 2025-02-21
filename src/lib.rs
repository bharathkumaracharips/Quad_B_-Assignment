use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{println, caller};
use std::cell::RefCell;

// Define the Wallet struct with balance and owner fields
#[derive(CandidType, Deserialize)]
struct Wallet {
    balance: u64,  // Store the wallet balance
    owner: Principal, // Store the owner's Principal ID
}

// Implement the Default trait for Wallet, initializing with zero balance
impl Default for Wallet {
    fn default() -> Self {
        Self {
            balance: 0,
            owner: Principal::anonymous(), // Default owner is anonymous
        }
    }
}

// Use thread-local storage to store the wallet state
thread_local! {
    static WALLET: RefCell<Wallet> = RefCell::new(Wallet::default());
}

// Initialize the wallet, setting the caller as the owner
#[ic_cdk::init]
fn init() {
    let caller_id = caller();
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        wallet.owner = caller_id; // Assign the caller as the owner
    });
}

// Query function to get the current balance of the wallet
#[ic_cdk::query]
fn get_balance() -> u64 {
    WALLET.with(|wallet| wallet.borrow().balance) // Return the balance
}

// Update function to receive tokens (only the owner can call this)
#[ic_cdk::update]
fn receive_tokens(amount: u64) {
    let caller_id = caller();
    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        if wallet.owner != caller_id {
            ic_cdk::trap("Unauthorized: Only the wallet owner can receive tokens");
        }
        wallet.balance += amount; // Add the received amount to balance
    });
}

// Update function to send tokens to another wallet
#[ic_cdk::update]
fn send_tokens(to: String, amount: u64) -> Result<(), String> {
    let caller_id = caller();
    let receiver_principal = Principal::from_text(&to)
        .map_err(|_| "Invalid Principal ID".to_string())?; // Validate recipient ID

    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    WALLET.with(|wallet| {
        let mut wallet = wallet.borrow_mut();
        if wallet.owner != caller_id {
            return Err("Unauthorized: Only the wallet owner can send tokens".to_string());
        }
        if wallet.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        wallet.balance -= amount; // Deduct amount from sender's balance
        Ok(())
    })?;

    println!("Sent {} tokens to {}", amount, receiver_principal.to_text());
    Ok(())
}

// Query function to get the caller's Principal ID
#[ic_cdk::query]
fn get_caller_principal() -> String {
    caller().to_text() // Return the caller's Principal ID as a string
}