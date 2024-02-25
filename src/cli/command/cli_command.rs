use async_trait::async_trait;

#[async_trait]
pub trait CliCommand {
    async fn execute(&self, args: Vec<String>);
}
