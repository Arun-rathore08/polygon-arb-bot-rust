use anyhow::Result;
use ethers::{
    abi::Address,
    contract::abigen,
    providers::{Http, Provider},
    types::U256,
};
use std::sync::Arc;

pub fn make_provider(rpc_url: &str) -> Result<Arc<Provider<Http>>> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    Ok(Arc::new(provider))
}

abigen!(UniswapV2Router, r#"[function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts)]"#);

pub struct RouterClient {
    pub router: UniswapV2Router<Provider<Http>>,
}

impl RouterClient {
    pub fn new(provider: &Arc<Provider<Http>>, router_addr: Address) -> Result<Self> {
        Ok(Self { router: UniswapV2Router::new(router_addr, provider.clone()) })
    }

    pub async fn get_amount_out(&self, amount_in: U256, token_in: Address, token_out: Address) -> Result<U256> {
        let path: Vec<Address> = vec![token_in, token_out];
        let amounts: Vec<U256> = self.router.get_amounts_out(amount_in, path).call().await?;
        Ok(*amounts.last().expect("router returned empty amounts"))
    }
}
