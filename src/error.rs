#[derive(Debug, Clone)]
pub enum LedgerError {
    AccountNotFound(u64),

    InsufficientFunds {
        account_id: u64,
        available: i64,
        requested: i64,
    },

    SelfTransfer(u64),
    InvalidAmount(u64),
    InvalidSignature(u64),
    InvalidNonce {
        account_id: u64,
        expected: u64,
        got: u64,
    },
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountNotFound(id) => {
                write!(f, "account {id} not found")
            }
            Self::InsufficientFunds {
                account_id,
                available,
                requested,
            } => {
                write!(
                    f,
                    "acccount {account_id}: insufficient funds \
                (available={available}, requested={requested})"
                )
            }
            Self::SelfTransfer(id) => {
                write!(f, "can not transfer to the same account ({id})")
            }
            Self::InvalidAmount(n) => {
                write!(f, "Amount must be positive, got {n}")
            }
            Self::InvalidSignature(id) => {
                write!(f, "account {id}: signature verification failed")
            }
            Self::InvalidNonce {
                account_id,
                expected,
                got,
            } => {
                write!(
                    f,
                    "account {account_id}: invalid nonce \
                     (expected={expected}, got={got})"
                )
            }
        }
    }
}

impl std::error::Error for LedgerError {}
