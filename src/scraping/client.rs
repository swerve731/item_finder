use fantoccini::ClientBuilder;
use port_check::free_local_port_in_range;
use serde_json::json;
use super::error::Error;
// const DEFAULT_PORT: u16 = 4444;


/// this is a wrapper for the fantoccini client so I can start new instances and kill them when done the query
pub struct OneTimeClient {
    pub client: fantoccini::Client,
    port: u16,
    process: tokio::process::Child,
}

impl Drop for OneTimeClient {
    fn drop(&mut self) {
        // kill the chromedriver port when done so the port can be reused
        println!("Killing chromedriver on port {}", self.port);
        let _process_kill = self
            .process
            .start_kill()
            .map_err(|e| {
                println!("Error killing port for onetime client: {}", e);
            })
            .expect("failed to kill chromedriver");

    }
}

impl OneTimeClient {
    pub async fn start_client() -> Result<Self, Error> {
        let port = free_local_port_in_range(4001..=15000)
            .ok_or(Error::NoOpenPorts)?;
        // let port = DEFAULT_PORT;
    
    
        let child = tokio::process::Command::new("chromedriver")
            .arg(format!("--port={}", port))
            // .kill_on_drop(true) // this doesnt work, probably because the process is dropped at the end of this function before it gets used
            .spawn()
            .map_err(|e|Error::ChromeDriver(e))?;


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
    
        println!("Starting chromedriver on port {}", port);
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
    
        Ok(OneTimeClient {
            client: c,
            port,
            process: child,
        })
    
    }    
}
