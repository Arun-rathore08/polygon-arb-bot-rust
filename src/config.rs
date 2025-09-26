use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use ethers::types::Address;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub rpc_url: String,
    pub dex_a_router: Address,
    pub dex_b_router: Address,
    pub token_in: Address,
    pub token_out: Address,
    pub token_in_decimals: u32,
    pub token_out_decimals: u32,
    pub trade_size_in_token_in: f64,
    pub gas_cost_usdc: f64,
    pub min_profit_usdc: f64,
    pub poll_interval_secs: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let rpc_url = req("RPC_URL")?;
        let dex_a_router = addr("DEX_A_ROUTER")?;
        let dex_b_router = addr("DEX_B_ROUTER")?;
        let token_in = addr("TOKEN_IN")?;
        let token_out = addr("TOKEN_OUT")?;
        let token_in_decimals = parse_u32("TOKEN_IN_DECIMALS", 18)?;
        let token_out_decimals = parse_u32("TOKEN_OUT_DECIMALS", 6)?;
        let trade_size_in_token_in = parse_f64("TRADE_SIZE_IN_TOKEN_IN", 1.0)?;
        let gas_cost_usdc = parse_f64("GAS_COST_USDC", 2.0)?;
        let min_profit_usdc = parse_f64("MIN_PROFIT_USDC", 5.0)?;
        let poll_interval_secs = parse_u32("POLL_INTERVAL_SECS", 20)?;

        Ok(Self {
            rpc_url, dex_a_router, dex_b_router, token_in, token_out,
            token_in_decimals, token_out_decimals, trade_size_in_token_in,
            gas_cost_usdc, min_profit_usdc, poll_interval_secs,
        })
    }

    pub fn redacted(&self) -> RedactedConfig {
        RedactedConfig {
            rpc_url: "<redacted>".into(),
            dex_a_router: self.dex_a_router,
            dex_b_router: self.dex_b_router,
            token_in: self.token_in,
            token_out: self.token_out,
            token_in_decimals: self.token_in_decimals,
            token_out_decimals: self.token_out_decimals,
            trade_size_in_token_in: self.trade_size_in_token_in,
            gas_cost_usdc: self.gas_cost_usdc,
            min_profit_usdc: self.min_profit_usdc,
            poll_interval_secs: self.poll_interval_secs,
        }
    }
}

#[derive(Debug)]
pub struct RedactedConfig {
    pub rpc_url: String,
    pub dex_a_router: Address,
    pub dex_b_router: Address,
    pub token_in: Address,
    pub token_out: Address,
    pub token_in_decimals: u32,
    pub token_out_decimals: u32,
    pub trade_size_in_token_in: f64,
    pub gas_cost_usdc: f64,
    pub min_profit_usdc: f64,
    pub poll_interval_secs: u32,
}

fn req(key: &str) -> Result<String> {
    Ok(env::var(key).with_context(|| format!("missing env {key}"))?)
}

fn addr(key: &str) -> Result<Address> {
    let s = req(key)?;
    let s = s.trim_start_matches("0x");
    let bytes = hex::decode(s).with_context(|| format!("invalid hex for {key}"))?;
    if bytes.len() != 20 { bail!("{key} must be 20 bytes (40 hex chars)"); }
    Ok(Address::from_slice(&bytes))
}

fn parse_u32(key: &str, default: u32) -> Result<u32> {
    Ok(env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default))
}

fn parse_f64(key: &str, default: f64) -> Result<f64> {
    Ok(env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default))
}
