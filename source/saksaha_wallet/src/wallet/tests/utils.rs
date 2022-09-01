use crate::wallet::Wallet;
use crate::{db::WalletDB, Config, CredentialManager, WalletCredential, RPC};
use std::sync::Arc;

use crate::rpc::tests::utils::MockWalletContext;
