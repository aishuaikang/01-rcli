pub mod cli;
pub mod process;
pub mod utils;

#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(&self) -> anyhow::Result<()>;
}
