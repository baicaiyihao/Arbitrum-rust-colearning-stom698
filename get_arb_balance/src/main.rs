use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, U256};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // 查询地址
    let address: Address = "0x94E43E9C8177a468ce00839657dD0562b242Ed50".parse()?;

    // 获取余额
    let balance: U256 = provider.get_balance(address).await?;

    // 转换为 ETH（除以 10^18）
    let balance_eth = balance.to_string().parse::<f64>()? / 1e18;

    println!("地址: {}", address);
    println!("余额: {} wei", balance);
    println!("余额: {:.6} ETH", balance_eth);

    Ok(())
}