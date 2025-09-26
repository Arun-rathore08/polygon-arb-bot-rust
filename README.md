# Polygon Arbitrage Opportunity Detector Bot (Rust)

This project implements a simple **arbitrage opportunity detector bot** on the **Polygon network** using **Rust**.  
The bot periodically checks prices for a chosen token pair (e.g., WETH/USDC) across two different Uniswap V2–compatible DEXes (e.g., QuickSwap and SushiSwap).  
If it finds a significant price difference, it calculates the simulated profit (after subtracting a simplified gas cost) and logs the opportunity into a local SQLite database.

⚠️ **Disclaimer:**  
This project is **for educational and research purposes only**.  
It does **not** execute trades or handle private keys. Quotes may not reflect actual executable prices due to slippage, liquidity depth, or MEV.  
Do not use this code in production without a full audit and additional risk management.

---

## ✨ Features

- **Multi-DEX price fetching**  
  Fetch token swap quotes from two Uniswap V2–compatible routers using `getAmountsOut`.

- **Arbitrage opportunity detection**  
  Compare swap quotes and flag if a profitable difference exists.

- **Simulated profit calculation**  
  Estimate profit for a fixed trade size, subtracting a simplified fixed gas cost in USDC.

- **Configuration management**  
  Configure RPC endpoint, router addresses, tokens, trade size, gas cost, and profit thresholds via `.env`.

- **SQLite logging**  
  Save every detected opportunity with timestamp, token pair, prices, and net profit to a local `arb.db`.

---

## 🧱 Architecture

Provider (Polygon RPC)
├── Router A (DEX A)
└── Router B (DEX B)
↓
Fetch swap quotes (TOKEN_IN -> TOKEN_OUT)
↓
Arbitrage Engine
- Compare quotes
- Calculate profit
- Check threshold
↓
Database Logger (SQLite)
↓
Console output

makefile
Copy code

---

## ⚙️ Configuration

The bot reads from a `.env` file. A sample is provided in `.env.example`:

```dotenv
# --- Chain / RPC ---
RPC_URL=https://polygon-rpc.com

# --- Routers ---
# QuickSwap V2 Router (verify): 0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff
# SushiSwap V2 Router (verify):  0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506
DEX_A_ROUTER=0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff
DEX_B_ROUTER=0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506

# --- Tokens ---
# WETH (18): 0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619
# USDC (6):  (verify actual Polygon USDC address)
TOKEN_IN=0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619
TOKEN_OUT=0x0000000000000000000000000000000000000000

TOKEN_IN_DECIMALS=18
TOKEN_OUT_DECIMALS=6

# --- Strategy ---
TRADE_SIZE_IN_TOKEN_IN=1.0     # Trade size in TOKEN_IN (e.g., 1 WETH)
GAS_COST_USDC=2.0              # Fixed gas cost assumption in USDC
MIN_PROFIT_USDC=5.0            # Only log if net profit >= threshold
POLL_INTERVAL_SECS=20          # Time between checks
▶️ Running the bot
Prerequisites
Rust (latest stable) → install via rustup

Access to a Polygon RPC endpoint (public or private)

SQLite (optional; installed by default on most Linux/macOS)

Steps
bash
Copy code
# 1. Clone or create project
git clone https://github.com/yourname/polygon-arb-opportunity-bot.git
cd polygon-arb-opportunity-bot

# 2. Copy and edit environment file
cp .env.example .env
nano .env   # fill in actual addresses and RPC URL

# 3. Build and run
cargo run --release
📊 Example console output
csharp
Copy code
Loaded config: AppConfig { ... }
[OPPORTUNITY] 2025-09-25T18:00:10Z | Buy on DEX A / Sell on DEX B | WETH->USDC | amount_in=1.0 | net=$7.32
No profitable opportunity this round.
No profitable opportunity this round.
[OPPORTUNITY] 2025-09-25T18:01:50Z | Buy on DEX B / Sell on DEX A | WETH->USDC | amount_in=1.0 | net=$5.12
📂 Database schema
Opportunities are stored in arb.db with this schema:

sql
Copy code
CREATE TABLE opportunities (
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
);
You can explore with:

bash
Copy code
sqlite3 arb.db "SELECT * FROM opportunities ORDER BY id DESC LIMIT 5;"
🚀 Potential Improvements
Real trade execution via private key signing and Polygon transactions

Multi-hop route support (e.g., TOKEN_IN → MATIC → USDC)

Uniswap V3 support with quoter contracts and tick math

Gas estimation from RPC instead of fixed USDC cost

Monitoring & alerts (e.g., Prometheus/Grafana integration, Telegram notifications)

📝 License
MIT © 2025 — Arun Rathore

🙏 Acknowledgements
ethers-rs for Rust Ethereum/Polygon integration

Uniswap V2 router ABI

Polygon network public RPC providers
