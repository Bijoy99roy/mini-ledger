use std::collections::HashMap;

use ed25519_dalek::Signature;

use crate::{Account, Keypair, LedgerError, Transaction, TxId, crypto};

pub struct Ledger {
    accounts: HashMap<u64, Account>,
    transactions: Vec<Transaction>,
    next_account_id: u64,
    nect_tx_id: u64,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: Vec::new(),
            next_account_id: 1,
            nect_tx_id: 1,
        }
    }

    pub fn generate_account(
        &mut self,
        name: impl Into<String>,
        inital_balance: i64,
    ) -> (u64, Keypair) {
        let keypair = Keypair::generate();

        let id = self.next_account_id;
        self.next_account_id += 1;

        let account = Account::new(id, name, inital_balance, keypair.public_key_bytes());
        self.accounts.insert(id, account);

        (id, keypair)
    }

    pub fn close_account(&mut self, id: u64) -> Result<Account, LedgerError> {
        self.accounts
            .remove(&id)
            .ok_or(LedgerError::AccountNotFound(id))
    }

    pub fn get_account(&self, id: u64) -> Option<&Account> {
        self.accounts.get(&id)
    }

    pub fn get_accounts(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    pub fn get_balance(&self, id: u64) -> Option<i64> {
        self.accounts.get(&id).map(|acc| acc.balance)
    }

    pub fn transfer(
        &mut self,
        from: u64,
        to: u64,
        amount: i64,
        nonce: u64,
        signature: &Signature,
    ) -> Result<TxId, LedgerError> {
        if from == to {
            return Err(LedgerError::SelfTransfer(from));
        }

        if amount <= 0 {
            return Err(LedgerError::InvalidAmount(amount));
        }

        let sender = self
            .accounts
            .get(&from)
            .ok_or(LedgerError::AccountNotFound(from))?;

        if nonce != sender.nonce {
            return Err(LedgerError::InvalidNonce {
                account_id: from,
                expected: sender.nonce,
                got: nonce,
            });
        }

        let message = crypto::transfer_message(from, to, amount, nonce);
        crypto::verify(&sender.public_key, &message, signature)
            .map_err(|_| LedgerError::InvalidSignature(from))?;

        if sender.balance < amount {
            return Err(LedgerError::InsufficientFunds {
                account_id: from,
                available: sender.balance,
                requested: amount,
            });
        }

        if !self.accounts.contains_key(&to) {
            return Err(LedgerError::AccountNotFound(to));
        }

        {
            let s = self.accounts.get_mut(&from).unwrap();
            s.balance -= amount;
            s.nonce += 1;
        }

        self.accounts.get_mut(&to).unwrap().balance += amount;

        let tx_id = TxId(self.nect_tx_id);
        self.nect_tx_id += 1;
        self.transactions
            .push(Transaction::new(tx_id, from, to, amount));

        Ok(tx_id)
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn transactions_for(&self, id: u64) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.from == id || tx.to == id)
            .collect()
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}
