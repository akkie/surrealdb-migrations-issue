use crate::SurrealDbTestContext;

#[tokio::test]
async fn test_one() -> anyhow::Result<()> {
    SurrealDbTestContext::init().await.unwrap();

    Ok(())
}
