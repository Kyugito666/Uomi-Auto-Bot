use ethers::prelude::*;
use ethers::utils::parse_units;
use std::sync::Arc;
use std::str::FromStr;
use crate::constants::*;

abigen!(
    ERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function deposit() external payable
        function withdraw(uint256 wad) external
    ]"#,
);

// ... (Tambahkan ABI untuk Router jika diperlukan) ...

pub type WalletClient = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn get_wallet_balance(client: &Arc<WalletClient>, addr: Address) -> eyre::Result<U256> {
    Ok(client.get_balance(addr, None).await?)
}

pub async fn wrap_uomi(client: &Arc<WalletClient>, amount_eth: &str) -> eyre::Result<TxHash> {
    let contract = ERC20::new(Address::from_str(WUOMI_ADDRESS)?, Arc::clone(client));
    let value = parse_units(amount_eth, "ether")?.into();

    let tx = contract.deposit().value(value);
    let pending_tx = tx.send().await?;
    Ok(*pending_tx)
}

pub async fn unwrap_wuomi(client: &Arc<WalletClient>, amount_eth: &str) -> eyre::Result<TxHash> {
    let contract = ERC20::new(Address::from_str(WUOMI_ADDRESS)?, Arc::clone(client));
    let value = parse_units(amount_eth, "ether")?.into();

    let tx = contract.withdraw(value);
    let pending_tx = tx.send().await?;
    Ok(*pending_tx)
}
// Fungsi swap dan add liquidity memerlukan ABI yang lebih kompleks dan logika encoding.
// Ini adalah contoh sederhana untuk wrap/unwrap. Implementasi penuh akan mengikuti pola yang sama.
