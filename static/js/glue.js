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

async function displayErrOrReload(response) {
    if (response.status !== 200) {
        const resp = await response.json();
        var errBox = document.getElementById("errBox");
        errBox.style.visibility = "visible";
        errBox.innerText = resp.message;
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

document.addEventListener("DOMContentLoaded", function() {
    var newNote = document.querySelector("#newNote");
    newNote.addEventListener("submit", addNewNote)

    document.getElementById("errBox").style.visibility = "hidden";
});

