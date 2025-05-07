function generateRandomNumberString(length) {
    let result = '';
    const characters = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    const charactersLength = characters.length;
    for (let i = 0; i < length; i++) {
      result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
  }
  

class ProductItem {
    constructor(title, price, imageUrl, productUrl, storeName, storeColor) {
        this.title = title;
        this.price = price;
        this.imageUrl = imageUrl;
        this.productUrl = productUrl;
        this.storeName = storeName;
        this.storeColor = storeColor;
    }
    static fromJson(json) {
        return new ProductItem(
            json.title,
            json.price,
            json.image_url,
            json.product_url,
            json.store_name,
            json.store_color

        );
    }

    static toComponent(productItem, canvasId) {
        // trim the title to 20 chars
        let title = productItem.title;
        if (title.length > 20) {
            title = title.substring(0, 20) + "...";
        }
        console.log("HERE");
        let price = productItem.price;
        price = price.toFixed(2);

        // <img src="${productItem.imageUrl}" alt="${productItem.title}">

        
        let html = `
            <div class="top">
                <canvas class="image" id="${canvasId}" ></canvas>
            </div>

            <div class="bottom">
                <h2>${title}</h2>
                <p>Price: ${price}$</p>
                <a target="_blank" style="background-color:${productItem.storeColor}" href="${productItem.productUrl}">View Product <i>on ${productItem.storeName}</i></a>
            </div>
        `;



        return html;
    }
}


class SearchError {
    constructor(message) {
        this.message = message;
    }

    static fromJson(json) {
        return new SearchError(
            json.error
        );
    }
}


// this is so the current stream can be interrupted if a new search is made
// this stops the current process from add elements after the user has searched for something else 
let active_streams = 0;
async function searchProducts() {
    active_streams++;
    console.log("searching products");
    let searchTerm = document.getElementById("term").value;
    const url = '/search?term=' + encodeURIComponent(searchTerm);
    const searchResultsDiv = document.getElementById("search-results");
    console.log("searching products with term: " + searchTerm);
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
        searchResultsDiv.innerHTML = ""; // Clear previous results
        while (true) {
            if (active_streams > 1) {

                console.log("stopping stream due to new search");
                active_streams--;
                break;
            }
            const { done, value } = await reader.read();

            if (done) {
                console.log('No more data in response.');
                break;
            }

            try {
                let canvasId = "canvas" + generateRandomNumberString(10);

                let chunk_str = decoder.decode(value, { stream: true });
                let chunk_json = JSON.parse(chunk_str);
                console.log(chunk_json)
                let productItem = ProductItem.fromJson(chunk_json);
                let component = ProductItem.toComponent(productItem, canvasId);
                
                const ItemDiv = document.createElement("div");
                ItemDiv.classList.add("product-item");
                ItemDiv.innerHTML = component;
                // console.log('Product Item:', productItem);
                // searchResultsDiv.innerHTML += component;
                searchResultsDiv.insertAdjacentElement('beforeend', ItemDiv);
                

                let image = new Image();
                image.src = productItem.imageUrl;
                image.width = 400;
                let canvas = document.getElementById(canvasId);

                image.onload = (function(canvasElement) {
                    return function() {
                        let ctx = canvasElement.getContext("2d");
                        canvasElement.setAttribute("src", productItem.imageUrl);
                        ctx.drawImage(image, 0, 0, 400, 200, 0, 0, 400, 200);
                    };
                })(canvas);

            } catch (e) {
                console.log(e)
                try {

                    let chunk_str = decoder.decode(value, { stream: true });
                    let chunk_json = JSON.parse(chunk_str);
                    let searchError = SearchError.fromJson(chunk_json);
                    console.error('Search Error:', searchError);0
                    continue
                }
                catch (e) {
                    console.error('Error parsing chunk:', e);
                }
            }
            // console.log('chunk:', decoder.decode(value, { stream: true }));
        } 
        active_streams--;

    } catch (error) {
        console.error('Error fetching search results:', error);
    }
}
