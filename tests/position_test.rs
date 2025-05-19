use bybit::prelude::*;

mod tests {
    use super::*;
    use tokio::test;

    static API_KEY: &str = ""; //Mockup string
    static SECRET: &str = ""; // Mockup string

    #[test]
    async fn position_info() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = PositionRequest::new(Category::Linear, Some("BTCUSDT"), None, None, None);
        match position.get_info(request).await {
            Ok(data) => println!("{:#?}", data),
            Err(e) => println!("{:#?}", e),
        }
    }

    #[test]
    async fn set_leverage() {
        let position: PositionManager = Bybit::new(Some(API_KEY.into()), Some(SECRET.into()));
        let request = LeverageRequest::new(Category::Linear, "BTCUSDT", 10);
        match position.set_leverage(request).await {
            Ok(data) => println!("{:?}", data),
            Err(e) => println!("{:?}", e),
        }
    }
}
