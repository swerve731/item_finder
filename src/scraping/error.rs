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
}


impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(e: scraper::error::SelectorErrorKind) -> Self {
        Error::Scraper {
            message: e.to_string(),
        }
    }
}