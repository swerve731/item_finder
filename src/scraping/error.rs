use derive_more::{Display, From};

#[derive(From, Debug, Display)]
#[display("There was a scraping error")]
pub enum Error {
    NotFound(String),
    WrongDataType(String),
    #[from]
    WebDriver(fantoccini::error::WebDriver),

    #[from]
    FantocciniCmd(fantoccini::error::CmdError),

    #[from]
    FantocciniSession(fantoccini::error::NewSessionError),
    #[from]
    Window(fantoccini::error::InvalidWindowHandle),

    #[from]
    #[display("Scraping selector error {message}")]
    Scraper{
        message: String,
    },
    #[from]
    #[display("could not start the client no open ports")]
    NoOpenPorts,

    #[from]
    #[display("could not start chromedriver, check if it is installed and the start command is chromedriver")]
    ChromeDriver(tokio::io::Error),
}


impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(e: scraper::error::SelectorErrorKind) -> Self {
        Error::Scraper {
            message: e.to_string(),
        }
    }
}