use ethers::prelude::*;

pub struct App {
    pub should_quit: bool,
    pub active_account_index: usize,
    pub wallets: Vec<Wallet<k256::ecdsa::SigningKey>>,
    pub logs: Vec<String>,
    pub balance: U256,
}

impl App {
    pub fn new(private_keys: Vec<String>) -> Self {
        let wallets: Vec<_> = private_keys
            .iter()
            .map(|pk| pk.parse::<LocalWallet>().unwrap().with_chain_id(Chain::Goerli)) // Ganti dengan Chain ID UOMI
            .collect();
        
        Self {
            should_quit: false,
            active_account_index: 0,
            wallets,
            logs: vec!["Selamat datang di UOMI Testnet Bot!".to_string()],
            balance: U256::zero(),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.should_quit = true,
            '1' => self.log("Aksi 1: Wrap UOMI..."),
            '2' => self.log("Aksi 2: Swap Random..."),
            // ... Tambahkan aksi lain ...
            _ => {}
        }
    }

    pub fn log(&mut self, message: &str) {
        self.logs.push(message.to_string());
    }

    pub fn get_active_wallet(&self) -> &Wallet<k256::ecdsa::SigningKey> {
        &self.wallets[self.active_account_index]
    }
}
