#[derive(Debug, Clone)]
pub struct Account {
    pub id: u64,
    pub balance: i64,
    pub public_key: [u8; 32],
    pub nonce: u64,
    pub name: String,
}

impl Account {
    pub fn new(id: u64, name: impl Into<String>, balance: i64, public_key: [u8; 32]) -> Self {
        Self {
            id,
            balance,
            public_key,
            nonce: 0,
            name: name.into(),
        }
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn address(&self) -> String {
        hex::encode(self.public_key)
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} | address={} | balance={} | nonce={}",
            self.id,
            self.name,
            &hex::encode(self.public_key),
            self.balance,
            self.nonce
        )
    }
}
