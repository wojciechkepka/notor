const BEARER = 'Bearer';
const TOKEN_EXPIRATION = 60;

function getBack() {
    window.history.back();
    location.reload();
}

function setCookie(name, value, minutes) {
    var expires = "";
    if (minutes) {
        var date = new Date();
        date.setTime(date.getTime() + (minutes * 60));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value || "")  + expires + "; path=/";
}

function getCookie(name) {
    var nameEq = name + "=";
    var chars = document.cookie.split(';');
    for (var i = 0; i < chars.length; i++) {
        var ch = chars[i];
        while (ch.charAt(0)==' ') ch = ch.substring(1, ch.length);
        if (ch.indexOf(nameEq) == 0) return c.substring(nameEq.length,c.length);
    }
    return null;
}

function request(method, ep, body = null, json = false) {
    var b = null;
    var h = {};
    if (body !== null) {
        if (json) {
            b = JSON.stringify(body);
            h["Content-Type"] = "application/json";
            h["Authorization"] = BEARER + " " + getCookie(BEARER)
        } else {
            b = body;
        }
    }

    return fetch(ep, {
        method: method,
        body: b,
        headers: h,
    });
}

function displayErr(message) {
    var errBox = document.getElementById("err_box");
    errBox.style.visibility = "visible";
    errBox.innerText = message;
}

async function displayErrOrReload(response) {
    if (response.status !== 200) {
        const resp = await response.json();
        displayErr(resp.message);
    } else {
        location.reload();
    }
}

async function deleteNote(id) {
    const response = await request("DELETE", "/notes/" + id);
    await displayErrOrReload(response);
}

async function addNewNote(event) {
    event.preventDefault();
    const data = new FormData(event.target);

    note = {
        title: data.get("title"),
        content: data.get("content"),
    };

    const response = await request("PUT", "/notes", body = note, json = true);
    await displayErrOrReload(response);
}

async function tagNote(event) {
    event.preventDefault();
    const data = new FormData(event.target);
    
    const id_re = /\/notes\/(\d+)/;
    const results = id_re.exec(window.location.href);
    if (!results) { displayErr("Failed to find note id in url"); return; }
    const note_id = results[1];
    const tag = data.get("tag");

    const response = await request("POST", "/notes/" + note_id + "/tags/" + tag);
    await displayErrOrReload(response);
}

async function handleLogin(event) {
    event.preventDefault();
    const data = new FormData(event.target);

    auth = {
        username: data.get("username"),
        pass: data.get("pass"),
    }

    const response = await request("POST", "/auth", body = auth, json = true);
    if (response.status !== 200) {
        const resp = await response.json();
        displayErr(resp.message);
    } else {
        const token = await response.text();
        setCookie(BEARER, token, TOKEN_EXPIRATION);
        location.replace("/web");
    }
}

async function handleHref(event) {
    event.preventDefault;
    console.log(event);
}


document.addEventListener("DOMContentLoaded", function() {
    var newNote = document.querySelector("#new_note");
    if (newNote) {
        newNote.addEventListener("submit", addNewNote);
    }

    var addTag = document.querySelector("#add_tag_form");
    if (addTag) {
        addTag.addEventListener("submit", tagNote);
    }

    var loginForm = document.querySelector("#login");
    if (loginForm) {
        loginForm.addEventListener("submit", handleLogin);
    }

    var links = document.querySelectorAll("a");
    for (var i = 0; i < links.length; i++) {
        links[i].addEventListener("onclick", handleHref);
    }

    document.getElementById("err_box").style.visibility = "hidden";
});

