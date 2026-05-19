use mini_ledger::{Ledger, transfer_message, verify};

fn main() {
    let mut ledger = Ledger::new();

    println!("Generate a new account");
    let (alice_id, alice_kp) = ledger.generate_account("Alice", 5000);
    let (bob_id, _) = ledger.generate_account("Bob", 3000);

    let alice = ledger.get_account(alice_id).unwrap();
    println!("Alice's account: {}", alice);

    println!("Address: {}", alice.address());

    println!("Pubkey: {}", alice_kp.public_key_hex());

    println!("Alice's balance: {:?}", ledger.get_balance(alice_id));
    println!("Bob's balance: {:?}", ledger.get_balance(bob_id));

    // Transfer funds
    let nonce = ledger.get_account(alice_id).unwrap().nonce();
    let msg = transfer_message(alice_id, bob_id, 1000, nonce);
    let sig = alice_kp.sign(&msg);
    let tx = ledger
        .transfer(alice_id, bob_id, 1000, nonce, &sig)
        .unwrap_or_else(|e| panic!("transfer failed: {e}"));
    println!("Alice transfered Bob 1000 : {tx}");

    // Signature verification
    println!("Verify signature");
    let nonce = ledger.get_account(alice_id).unwrap().nonce();
    let msg = transfer_message(alice_id, bob_id, 100, nonce);
    let sig = alice_kp.sign(&msg);

    match verify(&alice_kp.public_key_bytes(), &msg, &sig) {
        Ok(()) => println!("signature verified off-ledger"),
        Err(e) => println!("verification failed: {e}"),
    }

    // Try to verify a tampered msg
    let mut bad_msg = msg;
    bad_msg[0] ^= 0xFF;
    match verify(&alice_kp.public_key_bytes(), &bad_msg, &sig) {
        Ok(()) => println!("signature verified off-ledger"),
        Err(e) => println!("verification failed: {e}"),
    }

    // Print transactions
    for tx in ledger.transactions() {
        println!("  {tx}");
    }

    for tx in ledger.transactions_for(bob_id) {
        let dir = if tx.from == bob_id { "sent" } else { "recv" };
        println!("  [{dir}] {tx}");
    }

    // Close account
    match ledger.close_account(bob_id) {
        Ok(acc) => println!("closed: {acc}"),
        Err(e) => println!("error: {e}"),
    }

    // Attemp to close a closed account
    match ledger.close_account(bob_id) {
        Ok(acc) => println!("closed: {acc}"),
        Err(e) => println!("error: {e}"),
    }
}
