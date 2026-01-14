use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

// 定义 ERC20 合约 ABI
abigen!(
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address owner) external view returns (uint256)
    ]"#
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;
    let client = Arc::new(provider);

    // Arbitrum Sepolia 上的 USDC 测试代币
    let contract_address: Address = "0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d".parse()?;
    
    // 加载合约
    let contract = ERC20::new(contract_address, client.clone());

    println!("=== ERC20 合约信息查询 ===");
    println!("合约地址: {:?}\n", contract_address);

    // 调用只读方法
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;
    let total_supply = contract.total_supply().call().await?;

    println!("代币名称: {}", name);
    println!("代币符号: {}", symbol);
    println!("精度: {}", decimals);
    println!("总供应量: {}", total_supply);

    // 查询某地址余额
    let holder: Address = "0x94E43E9C8177a468ce00839657dD0562b242Ed50".parse()?;
    let balance = contract.balance_of(holder).call().await?;
    
    println!("\n=== 余额查询 ===");
    println!("地址: {:?}", holder);
    println!("余额: {} {}", ethers::utils::format_units(balance, decimals as u32)?, symbol);

    Ok(())
}