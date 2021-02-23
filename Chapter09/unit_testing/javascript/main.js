if (localStorage.getItem("user-token") == null) {
    window.location.replace(document.location.origin + "/login/");
} else {
    let cachedData = Date.parse(localStorage.getItem("item-cache-date"));
    let now = new Date();
    let difference = Math.round((now - cachedData) / (1000));

    if (difference <= 120) {
        runRenderProcess(JSON.parse(localStorage.getItem("item-cache-data")));
    } else {
        getItems();
    }
}


/**
 * Fires a series of render processes to fill out the main view.
 *
 * @param data {JSON} data from the API on the to-do items
 * @returns null
 */
function runRenderProcess(data) {
    renderItems(data["pending_items"], "edit", "pendingItems", editItem);
    renderItems(data["done_items"], "delete", "doneItems", deleteItem);
    document.getElementById("completeNum").innerHTML = data["done_item_count"];
    document.getElementById("pendingNum").innerHTML = data["pending_item_count"];
}


/**
 * Renders the to do items from the backend into a HTML div.
 *
 * @param items {Array} - list of to do items
 * @param processType {String} - the type of process that the button belonging to the to do item
 * @param elementId {String} - the id of the HTML element that the items will be inserted
 * @param processFunction {editItem | deleteItem} - function that is fired once the button is clicked
 */
function renderItems(items, processType,
                     elementId, processFunction) {
    let placeholder = "<div>"
    let itemsMeta = [];

    for (i = 0; i < items.length; i++) {
        let title = items[i]["title"];
        let placeholderId = processType +
            "-" + title.replaceAll(" ", "-");

        placeholder += '<div class="itemContainer">' +
            '<p>' + title + '</p>' +
            '<div class="actionButton" ' + 'id="' + placeholderId + '">'
            + processType + '</div>' + "</div>";
        itemsMeta.push({"id": placeholderId, "title": title});
    }
    placeholder += "</div>"
    document.getElementById(elementId).innerHTML = placeholder;

    for (i = 0; i < itemsMeta.length; i++) {
        document.getElementById(
            itemsMeta[i]["id"]).addEventListener(
            "click", processFunction);
    }
}


/**
 * Packages an API call ready to be sent.
 *
 * @param url {String} - the URL endpoint for the API call
 * @param method {String} - the method of the API call => POST, GET, PUT
 * @returns {XMLHttpRequest} - the API packaged API request
 */
function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function() {
        if (this.readyState === this.DONE) {
            if (this.status === 401) {
                window.location.replace(document.location.origin + "/login/");
            } else {
                runRenderProcess(JSON.parse(this.responseText));
                localStorage.setItem("item-cache-date", new Date());
                localStorage.setItem("item-cache-data", this.responseText);
            }
        }
    });
    xhr.open(method, "/api/v1" + url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', localStorage.getItem("user-token"));
    return xhr
}


/**
 * Gets the title from this, and calls the edit API endpoint.
 */
function editItem() {
    let title = this.id.replaceAll("-", " ").replace("edit ", "");
    let call = apiCall("/item/edit", "PUT");
    let json = {
        "title": title,
        "status": "done"
    };
    call.send(JSON.stringify(json));
}


/**
 * Gets the title from this, and calls the delete API endpoint.
 */
function deleteItem() {
    let title = this.id.replaceAll("-", " ").replace("delete ", "");
    let call = apiCall("/item/delete", "POST");
    let json = {
        "title": title,
        "status": "done"
    };
    call.send(JSON.stringify(json));
}


/**
 * Calls the get items API.
 */
function getItems() {
    let call = apiCall("/item/get", 'GET');
    call.send()
}

document.getElementById("create-button").addEventListener(
    "click", createItem);


/**
 * Gets the title from the HTML with "name" as ID, and calls the create API endpoint with it.
 */
function createItem() {
    let title = document.getElementById("name");
    let call = apiCall("/item/create/" + title.value, "POST");
    call.send();
    document.getElementById("name").value = null;
}