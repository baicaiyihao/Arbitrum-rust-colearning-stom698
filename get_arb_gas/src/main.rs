use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, U256};
use alloy::rpc::types::TransactionRequest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let from: Address = "0x94E43E9C8177a468ce00839657dD0562b242Ed50".parse()?;
    let to: Address = "0xB123d8CEb0b241f78f6452dA848F4FBA1a22FeaF".parse()?;

    let tx = TransactionRequest::default()
        .from(from)
        .to(to)
        .value(U256::from(1_000_000_000_000_000_u64)); // 0.001 ETH

    let gas_limit = provider.estimate_gas(tx).await?;
    let gas_price = provider.get_gas_price().await?;

    let gas_cost_wei = U256::from(gas_limit) * U256::from(gas_price);
    let gas_price_gwei = gas_price as f64 / 1e9;
    let gas_cost_eth = gas_cost_wei.to_string().parse::<f64>()? / 1e18;

    println!("From: {}", from);
    println!("To: {}", to);
    println!("Gas Limit: {}", gas_limit);
    println!("Gas Price: {:.4} Gwei", gas_price_gwei);
    println!("预估费用: {} wei", gas_cost_wei);
    println!("预估费用: {:.10} ETH", gas_cost_eth);

    Ok(())
}