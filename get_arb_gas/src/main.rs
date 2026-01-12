use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::U256;
use std::error::Error;

const BASE_TRANSFER_GAS_LIMIT: u64 = 21000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 动态获取实时 Gas 价格
    let gas_price = provider.get_gas_price().await?;

    // 计算预估 Gas 费
    let gas_fee = estimate_transfer_gas_fee(gas_price);

    let gas_price_gwei = gas_price as f64 / 1e9;
    let gas_fee_eth = gas_fee.to_string().parse::<f64>()? / 1e18;

    println!("=== Arbitrum Sepolia Gas 估算 ===");
    println!("Gas 价格: {:.4} Gwei", gas_price_gwei);
    println!("Gas 限额: {} (基础转账)", BASE_TRANSFER_GAS_LIMIT);
    println!("预估 Gas 费: {:.10} ETH", gas_fee_eth);

    Ok(())
}

fn estimate_transfer_gas_fee(gas_price: u128) -> U256 {
    U256::from(gas_price) * U256::from(BASE_TRANSFER_GAS_LIMIT)
}