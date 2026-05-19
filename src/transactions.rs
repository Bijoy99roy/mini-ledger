use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct TxId(pub u64);

impl std::fmt::Display for TxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TxId,

    pub from: u64,
    pub to: u64,
    pub amount: i64,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(id: TxId, from: u64, to: u64, amount: i64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            id,
            from,
            to,
            amount,
            timestamp,
        }
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | from={} to={} amount={} ts={}",
            self.id, self.from, self.to, self.amount, self.timestamp
        )
    }
}
