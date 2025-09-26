use anyhow::Result;
use dotenvy::dotenv;
use tokio::time::{sleep, Duration};

mod config;
mod dex;
mod arbitrage;
mod db;

use crate::arbitrage::{ArbEngine, QuotePair};
use crate::config::AppConfig;
use crate::db::Db;
use crate::dex::RouterClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cfg = AppConfig::from_env()?;
    println!("Loaded config: {:?}", cfg.redacted());

    // Init DB
    let db = Db::new("arb.db")?;
    db.init()?;

    // Provider and routers
    let provider = dex::make_provider(&cfg.rpc_url)?;
    let router_a = RouterClient::new(&provider, cfg.dex_a_router)?;
    let router_b = RouterClient::new(&provider, cfg.dex_b_router)?;

    let engine = ArbEngine::new(cfg.clone());

    loop {
        match tick_once(&engine, &router_a, &router_b).await {
            Ok(Some(record)) => {
                println!("[OPPORTUNITY] {}", record);
                if let Err(e) = db.insert_opportunity(&record) {
                    eprintln!("DB insert error: {e:?}");
                }
            }
            Ok(None) => {
                println!("No profitable opportunity this round.");
            }
            Err(e) => eprintln!("Tick error: {e:?}"),
        }

        sleep(Duration::from_secs(engine.cfg.poll_interval_secs as u64)).await;
    }
}

async fn tick_once(
    engine: &ArbEngine,
    a: &RouterClient,
    b: &RouterClient,
) -> Result<Option<db::OpportunityRecord>> {
    let amount_in = engine.amount_in_base_units()?;

    let q_a = a
        .get_amount_out(amount_in, engine.cfg.token_in, engine.cfg.token_out)
        .await?;
    let q_b = b
        .get_amount_out(amount_in, engine.cfg.token_in, engine.cfg.token_out)
        .await?;

    let quotes = QuotePair {
        dex_a_out: q_a,
        dex_b_out: q_b,
    };

    if let Some(op) = engine.evaluate(quotes)? {
        let rec = db::OpportunityRecord::from_eval(&op, &engine.cfg);
        return Ok(Some(rec));
    }
    Ok(None)
}

