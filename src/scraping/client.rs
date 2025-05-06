use fantoccini::ClientBuilder;
use serde_json::json;

pub async fn default_client() -> Result<fantoccini::Client, super::error::Error> {

    // this is how I passed the stockX captia   
    // removes default headless chrome flags
    let mut caps = serde_json::Map::new();
    caps.insert(
        "goog:chromeOptions".to_string(),
        json!({
            "args": [
                "--headless",
                "--disable-gpu",
                "--disable-blink-features=AutomationControlled",
                "--no-sandbox",
                "--window-size=1920,1080"
            ],
            "excludeSwitches": ["enable-automation"],
            "useAutomationExtension": false
        }),
    );
    let c = ClientBuilder::native()
        .capabilities(caps.into())
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // when a service checks if the browser is a web driver it calls this function instead of the google one
    let script = r#"
      Object.defineProperty(navigator, 'webdriver', {
        get: () => false
      });
    "#;
    c.execute(script, Vec::new()).await?;

    Ok(c)
}


