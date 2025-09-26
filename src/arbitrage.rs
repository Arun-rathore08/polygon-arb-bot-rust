use anyhow::Result;
use ethers::types::U256;
use crate::config::AppConfig;

#[derive(Clone, Copy, Debug)]
pub struct QuotePair {
    pub dex_a_out: U256,
    pub dex_b_out: U256,
}

#[derive(Debug)]
pub struct Evaluated {
    pub buy_on: &'static str,
    pub sell_on: &'static str,
    pub amount_out_buy: U256,
    pub amount_out_sell: U256,
    pub gross_profit_out: f64,
    pub net_profit_out: f64,
}

pub struct ArbEngine {
    pub cfg: AppConfig,
}

impl ArbEngine {
    pub fn new(cfg: AppConfig) -> Self { Self { cfg } }

    pub fn amount_in_base_units(&self) -> Result<U256> {
        let mul = 10u128.pow(self.cfg.token_in_decimals) as f64;
        let raw = (self.cfg.trade_size_in_token_in * mul).round();
        Ok(U256::from(raw as u128))
    }

    pub fn evaluate(&self, quotes: QuotePair) -> Result<Option<Evaluated>> {
        let (buy_on, sell_on, out_buy, out_sell) = if quotes.dex_a_out > quotes.dex_b_out {
            ("DEX B", "DEX A", quotes.dex_b_out, quotes.dex_a_out)
        } else if quotes.dex_b_out > quotes.dex_a_out {
            ("DEX A", "DEX B", quotes.dex_a_out, quotes.dex_b_out)
        } else { return Ok(None); };

        let gross_profit = u256_to_float(out_sell - out_buy, self.cfg.token_out_decimals);
        let net_profit = gross_profit - self.cfg.gas_cost_usdc;

        if net_profit >= self.cfg.min_profit_usdc {
            Ok(Some(Evaluated { buy_on, sell_on, amount_out_buy: out_buy, amount_out_sell: out_sell,
                                gross_profit_out: gross_profit, net_profit_out: net_profit }))
        } else { Ok(None) }
    }
}

fn u256_to_float(v: U256, decimals: u32) -> f64 {
    let s = v.to_string();
    if decimals == 0 { return s.parse::<f64>().unwrap_or(0.0) }
    if s.len() <= decimals as usize {
        let mut out = String::from("0.");
        out.push_str(&"0".repeat(decimals as usize - s.len()));
        out.push_str(&s);
        return out.parse::<f64>().unwrap_or(0.0)
    }
    let idx = s.len() - decimals as usize;
    format!("{}.{}", &s[..idx], &s[idx..]).parse::<f64>().unwrap_or(0.0)
}
