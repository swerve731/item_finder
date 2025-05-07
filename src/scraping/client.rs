use fantoccini::ClientBuilder;
use serde_json::json;
use super::error::Error;
const DEFAULT_PORT: u16 = 4444;


pub async fn start_client() -> Result<fantoccini::Client, Error> {
    // let port = free_local_port_in_range(4000..=15000)
    //     .ok_or(Error::NoOpenPorts)?;
    let port = DEFAULT_PORT;

    // I tried this it starts a new chromedriver instance so each query has its own browser and does not have to wait for it to be free
    // but it has no effect on the speed of the scraping
    // and the chromedriver can handle multiple connections
    // let _res = tokio::process::Command::new("chromedriver")
    //     .arg(format!("--port={}", port))
    //     .spawn()
    //     .map_err(|e|Error::ChromeDriver(e))?;
 
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
        
        .connect(&format!("http://localhost:{}/session", port))
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
