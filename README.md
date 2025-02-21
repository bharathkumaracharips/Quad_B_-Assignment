# Quad_B_-Assignment

ICP Token Wallet

Overview

This project is an ICP Token Wallet implemented using Rust and the Internet Computer (IC) blockchain. The wallet allows users to:

Check their token balance

Receive tokens

Send tokens to other wallet addresses

Features

Balance Inquiry: Users can check their current token balance.

Token Transfer: Users can send tokens to other wallet addresses.

Token Reception: Users can receive tokens and update their balance.

Decentralized Deployment: The wallet is deployed on the Internet Computer.

Prerequisites


Before running the project, ensure you have the following installed:

DFX SDK (Dfinity’s command-line tool)

Install DFX (Internet Computer SDK)
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)”//macOS

Rust and Cargo
Install Rust: 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Wasm target for Rust (wasm32-unknown-unknown)

Git (for version control)

Installation

Clone the repository:

git clone https://github.com/bharathkumaracharips/Quad_B_-Assignment.git
cd Quad_B_-Assignment

Setup & Deployment

1. Start the Local Internet Computer Network

dfx start --background

2. Create the Canister

dfx canister create icp_token_wallet

3. Build the Project

dfx build

4. Deploy the Canister

dfx deploy

Usage manuall

Check Wallet Balance

dfx canister call icp_token_wallet get_balance

Receive Tokens

dfx canister call icp_token_wallet receive_tokens '(100)'

Send Tokens

dfx canister call icp_token_wallet send_tokens '("recipient_principal_id", 50)'

visit the local host 
and can work on it 

Troubleshooting

If you encounter errors such as "Only controllers of canister can call install_code", try updating the controller:

dfx canister update-settings icp_token_wallet --add-controller <your_principal_id>



Author
PS BHARATH KUMAR ACHARI

Contact
Email:- bharathkumaracharips@gmail.com
Portfolio :- https://bharathkumarachari.xyz/

