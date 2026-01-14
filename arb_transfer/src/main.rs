use ethers::prelude::*;
use std::env;
use std::sync::Arc;
use std::error::Error;

const BASE_TRANSFER_GAS_LIMIT: u64 = 200000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let private_key = env::var("PRIVATE_KEY")
        .expect("请设置环境变量 PRIVATE_KEY");

    let provider = Provider::<Http>::try_from("https://sepolia-rollup.arbitrum.io/rpc")?;
    let chain_id = provider.get_chainid().await?.as_u64();

    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    let from = wallet.address();
    let to: Address = "0xB123d8CEb0b241f78f6452dA848F4FBA1a22FeaF".parse()?;
    let amount = U256::from(1_000_000_000_000_000_u64); // 0.001 ETH

    println!("=== 地址校验 ===");
    println!("发送方: {:?}", from);
    println!("接收方: {:?}", to);

    let balance = provider.get_balance(from, None).await?;
    println!("发送方余额: {} ETH", ethers::utils::format_ether(balance));

    // 获取 EIP-1559 费用
    let (max_fee, priority_fee) = provider.estimate_eip1559_fees(None).await?;

    println!("\n=== Gas 费设置 (EIP-1559) ===");
    println!("Max Fee: {} Gwei", ethers::utils::format_units(max_fee, "gwei")?);
    println!("Priority Fee: {} Gwei", ethers::utils::format_units(priority_fee, "gwei")?);
    println!("Gas 限额: {}", BASE_TRANSFER_GAS_LIMIT);

    let gas_fee = max_fee * BASE_TRANSFER_GAS_LIMIT;
    println!("最大 Gas 费: {} ETH", ethers::utils::format_ether(gas_fee));

    if balance < amount + gas_fee {
        return Err("余额不足".into());
    }

    // 使用 EIP-1559 交易
    let tx = Eip1559TransactionRequest::new()
        .to(to)
        .value(amount)
        .gas(BASE_TRANSFER_GAS_LIMIT)
        .max_fee_per_gas(max_fee)
        .max_priority_fee_per_gas(priority_fee);

    println!("\n=== 发送交易 ===");
    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = pending_tx.tx_hash();
    println!("交易哈希: {:?}", tx_hash);

    println!("等待确认...");
    let receipt = pending_tx.await?.expect("交易失败");

    println!("\n=== 交易完成 ===");
    println!("区块号: {:?}", receipt.block_number);
    println!("Gas 使用: {:?}", receipt.gas_used);
    println!("状态: 成功 ✓");
    println!("\n查看: https://sepolia.arbiscan.io/tx/{:?}", tx_hash);

    Ok(())
}