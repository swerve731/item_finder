# Item finder 

A web scraping tool built with Rust that fetches product information (title, images and price) from StockX and eBay, and more in the future. based on a search term. It uses `fantoccini` for browser automation to handle dynamic content and `scraper` for parsing the HTML structure.

## Features

*   Scrapes product search results from:
    *   StockX
    *   eBay
    *   more coming soon...
*   Extracts product title, image, price.
*   Runs searches concurrently for faster results.
*   Configured to bypass some common bot detection mechanisms.

## Prerequisites

Before you can run this project, make sure you have the following installed and running:

1.  **Rust:** The project is built with Rust. If you don't have it installed, you can get it via rustup.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **ChromeDriver:** This project uses `fantoccini` which requires a WebDriver instance.
    *   Download the ChromeDriver executable that matches your installed Google Chrome version from the official ChromeDriver website.
    *   https://developer.chrome.com/docs/chromedriver/get-started
