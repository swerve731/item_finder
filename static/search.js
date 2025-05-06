class ProductItem {
    constructor(title, price, imageUrl, productUrl) {
        this.title = title;
        this.price = price;
        this.imageUrl = imageUrl;
        this.productUrl = productUrl;
    }
    static fromJson(json) {
        return new ProductItem(
            json.title,
            json.price,
            json.image_url,
            json.product_url
        );
    }

    static toComponent(productItem) {
        // trim the title to 20 chars
        let title = productItem.title;
        if (title.length > 20) {
            title = title.substring(0, 20) + "...";
        }
    
        let price = productItem.price;
        price = price.toFixed(2);

        return `
            <div class="product-item">
                <div class="top">
                    <img src="${productItem.imageUrl}" alt="${productItem.title}">
                </div>
                <div class="bottom">
                    <h2>${title}</h2>
                    <p>Price: ${price}$</p>
                    <a href="${productItem.productUrl}">View Product <i>on StockX</i></a>
                </div>
            </div>
        `;
    }
}

async function searchProducts() {
    console.log("searching products");
    let searchTerm = document.getElementById("term").value;
    const url = '/search?term=' + encodeURIComponent(searchTerm);
    const searchResultsDiv = document.getElementById("search-results");
    console.log("searching products with term: " + searchTerm);
    searchResultsDiv.innerHTML = ""; // Clear previous results
    try {
        
        const response = await fetch(url, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ 
                "term": searchTerm,
            })
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        if (!response.body) {
            console.log('Response body is not available.');
            return;
        }

        const reader = response.body.getReader();
        const decoder = new TextDecoder(); 

        while (true) {
            const { done, value } = await reader.read();

            if (done) {
                console.log('No more data in response.');
                break;
            }
            let chunk_str = decoder.decode(value, { stream: true });
            let chunk_json = JSON.parse(chunk_str);
            let productItem = ProductItem.fromJson(chunk_json);
            let component = ProductItem.toComponent(productItem);
            console.log('Product Item:', productItem);
            searchResultsDiv.innerHTML += component;
            // console.log('chunk:', decoder.decode(value, { stream: true }));
        }
    } catch (error) {
        console.error('Error fetching search results:', error);
    }
}
