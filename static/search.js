// import '/static/stopwatch.js'

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
        // console.log("HERE");
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
    console.log("searching products");
    active_streams++;

    let result_count = 0;
    let resultCountDiv = document.getElementById("result-count");

    let searchTerm = document.getElementById("term").value;
    let store_filter_elements = document.getElementsByClassName("store-filter");

    let stores = [];
    for (let i = 0; i < store_filter_elements.length; i++) {
        if (store_filter_elements[i].checked) {
            stores.push(store_filter_elements[i].value);
        }
    }


    const url = '/search';
    const searchResultsDiv = document.getElementById("search-results");
    console.log("searching products with term: " + searchTerm);

    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ 
                "term": searchTerm,
                "stores": stores
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

        searchResultsDiv.innerHTML = "";

        
        let stopwatch = new Stopwatch("stopwatch");

        stopwatch.start()
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

                // get and format data
                let chunk_str = decoder.decode(value, { stream: true });
                let chunk_json = JSON.parse(chunk_str);
                let productItem = ProductItem.fromJson(chunk_json);
                let component = ProductItem.toComponent(productItem, canvasId);
                const ItemDiv = document.createElement("div");

                // add items to the dom
                ItemDiv.classList.add("product-item");
                ItemDiv.innerHTML = component;
                searchResultsDiv.insertAdjacentElement('beforeend', ItemDiv);


                // render image
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

                result_count++;
                resultCountDiv.innerHTML = result_count;

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
        stopwatch.stop()
    } catch (error) {
        console.error('Error fetching search results:', error);
    }
}


// class LiveStopWatch {
//     constructor(target_id) {
//         this.target_id = target_id
//         this.is_running = false
//     }

//     async start(this) {
//         this.is_running = true;
//         let target_element = document.getElementById(this.target_id);

//         let start_time = Date.now();

//          while (this.is_running){
//             let elapsed_time = start_time - Date.now();
//             console.log(elapsed_time)
//             target_element.innerHTML = elapsed_time
//         }
//     }
// }