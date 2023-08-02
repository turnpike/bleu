use appbase::prelude::*;
use plugin::{
	ethereum_block::EthereumBlockPlugin, ethereum_tx_receipt::EthereumTxReceiptPlugin,
	sync::SyncRpcPlugin
};

mod error;
mod libs;
mod plugin;
mod types;

fn main() {
	env_logger::init();
	APP.register::<EthereumTxReceiptPlugin>();
	APP.register::<EthereumBlockPlugin>();
	APP.register::<SyncRpcPlugin>();
	APP.init();
	APP.plugin_init::<EthereumTxReceiptPlugin>();
	APP.plugin_init::<EthereumBlockPlugin>();
	APP.plugin_init::<SyncRpcPlugin>();
	APP.startup();
	APP.execute();
}
