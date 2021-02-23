const loginButton = document.getElementById('loginButton');
const username = document.getElementById('defaultLoginFormUsername');
const password = document.getElementById('defaultLoginFormPassword');
const message = document.getElementById("loginMessage");


loginButton.addEventListener("click", () => {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/api/v1/auth/login", true);
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onreadystatechange = function () {

        if (xhr.readyState === 4) {
            if (xhr.status === 200) {
                let token = xhr.getResponseHeader("token");
                localStorage.setItem("user-token", token);
                window.location.replace(document.location.origin);
            } else {
                message.innerText = "login failed please try again";
            }
        }
    };
    let data = JSON.stringify({"username": username.value,
                                     "password": password.value});
    xhr.send(data);
    message.innerText = "logging in"
})
