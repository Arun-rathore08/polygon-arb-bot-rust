use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use ethers::types::U256;
use std::fmt;

use crate::config::AppConfig;
use crate::arbitrage::Evaluated;

pub struct Db { conn: Connection }

impl Db {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self { conn: Connection::open(path)? })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ts_utc TEXT NOT NULL,
            dex_buy TEXT NOT NULL,
            dex_sell TEXT NOT NULL,
            token_pair TEXT NOT NULL,
            amount_in TEXT NOT NULL,
            quote_buy_out TEXT NOT NULL,
            quote_sell_out TEXT NOT NULL,
            gross_profit_usdc REAL NOT NULL,
            gas_cost_usdc REAL NOT NULL,
            net_profit_usdc REAL NOT NULL
        );"#)?;
        Ok(())
    }

    pub fn insert_opportunity(&self, r: &OpportunityRecord) -> Result<()> {
        self.conn.execute(r#"
        INSERT INTO opportunities (
          ts_utc,dex_buy,dex_sell,token_pair,amount_in,quote_buy_out,
          quote_sell_out,gross_profit_usdc,gas_cost_usdc,net_profit_usdc
        ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)"#,
        params![r.ts_utc,r.dex_buy,r.dex_sell,r.token_pair,r.amount_in,
                r.quote_buy_out,r.quote_sell_out,r.gross_profit_usdc,
                r.gas_cost_usdc,r.net_profit_usdc])?;
        Ok(())
    }
}

pub struct OpportunityRecord {
    pub ts_utc: String,
    pub dex_buy: String,
    pub dex_sell: String,
    pub token_pair: String,
    pub amount_in: String,
    pub quote_buy_out: String,
    pub quote_sell_out: String,
    pub gross_profit_usdc: f64,
    pub gas_cost_usdc: f64,
    pub net_profit_usdc: f64,
}

impl OpportunityRecord {
    pub fn from_eval(ev: &Evaluated, cfg: &AppConfig) -> Self {
        let ts = Utc::now().to_rfc3339();
        let token_pair = format!("{:?}->{:?}", cfg.token_in, cfg.token_out);
        Self {
            ts_utc: ts,
            dex_buy: ev.buy_on.into(),
            dex_sell: ev.sell_on.into(),
            token_pair,
            amount_in: cfg.trade_size_in_token_in.to_string(),
            quote_buy_out: ev.amount_out_buy.to_string(),
            quote_sell_out: ev.amount_out_sell.to_string(),
            gross_profit_usdc: ev.gross_profit_out,
            gas_cost_usdc: cfg.gas_cost_usdc,
            net_profit_usdc: ev.net_profit_out,
        }
    }
}

impl fmt::Display for OpportunityRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | Buy {} / Sell {} | {} | net=${:.4}",
               self.ts_utc,self.dex_buy,self.dex_sell,self.token_pair,self.net_profit_usdc)
    }
}
