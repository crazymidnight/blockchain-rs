use ring::digest::{Context, Digest, SHA256};
use ring::error::Unspecified;
use chrono::prelude::*;
use uuid::Uuid;

fn sha256_digest(data: String) -> Result<Digest, Unspecified> {
    let mut context = Context::new(&SHA256);
    context.update(&data.as_bytes());
    Ok(context.finish())
}

struct Transaction {
    from_address: String,
    to_address: String,
    amount: f64,
    timestamp: i64,
    transaction_id: String,
    signature: String,
}

impl Transaction {
    fn get_data_string(self: &Self) -> String {
        format!(
            "{}{}{}{}{}{}",
            self.from_address,
            self.to_address,
            self.amount.to_string(),
            self.timestamp.to_string(),
            self.transaction_id,
            self.signature,
        )
    }

    fn calculate_hash(self: &Self) -> String {
        format!("{:?}", sha256_digest(self.get_data_string()).unwrap())
    }

    fn is_valid(self: &Self) -> bool {
        if self.from_address == "" {
            return true
        }
        if self.signature == "" {
            return false
        }
        true
    }

    fn create(from_address: String, to_address: String, amount: f64) -> Transaction {
        Transaction{
            from_address,
            to_address,
            amount,
            transaction_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp_millis(),
            signature: String::from("123"),
        }
    }
}

fn main() {
    let transaction = Transaction::create(
        String::from("a"),
        String::from("b"),
        123.32,
    );
    println!("{}", transaction.get_data_string());
    println!("{}", transaction.calculate_hash());
}
