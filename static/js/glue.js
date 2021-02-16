function getBack() {
    window.history.back();
    location.reload();
}

function request(method, ep, body = null, json = false) {
    var b = null;
    var h = {};
    if (body !== null) {
        if (json) {
            b = JSON.stringify(body);
            h["Content-Type"] = "application/json";
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
    var errBox = document.getElementById("errBox");
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

document.addEventListener("DOMContentLoaded", function() {
    var newNote = document.querySelector("#newNote");
    if (newNote) {
        newNote.addEventListener("submit", addNewNote);
    }

    var addTag = document.querySelector("#addTagForm");
    if (addTag) {
        addTag.addEventListener("submit", tagNote);
    }

    document.getElementById("errBox").style.visibility = "hidden";
});

