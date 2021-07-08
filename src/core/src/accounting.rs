use serde::{Serialize, Deserialize};

use crate::db::DBId;

#[derive(Serialize, Deserialize)]
pub struct Account {
    balances: Vec<DBId<Balance>>
}

#[derive(Serialize, Deserialize)]
struct Balance {
    id: DBId<Balance>,
    amount: MoneyAmount,
    transactions: Vec<DBId<Transaction>>
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    id: DBId<Transaction>,
    from: (Balance, MoneyAmount),
    to: (Balance, MoneyAmount)
}

#[derive(Serialize, Deserialize)]
pub struct MoneyAmount {
    currency: Currency,
    amount: f64
}

#[derive(Serialize, Deserialize)]
pub enum Currency {
    EUR,
    Dollar
}
