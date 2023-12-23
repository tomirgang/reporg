use thirtyfour::prelude::*;

#[tokio::test]
async fn open_landing_page() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("http://127.0.0.1:8001/").await?;
    assert_eq!(driver.title().await?, "RepOrg");

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
