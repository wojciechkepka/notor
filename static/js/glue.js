
async function deleteNote(id) {
    const response = await fetch('/notes/' + id, {
        method: 'DELETE',
    });
}

async function addNewNote(event) {
    event.preventDefault();

    const data = new FormData(event.target);

    note = {
        title: data.get('title'),
        content: data.get('content'),
    };

    const response = await fetch('/notes', {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(note),
    });

    if (response.status !== 200) {
        const resp = await response.json();
        var errBox = document.getElementById("newNoteErr");
        errBox.style.visibility = 'visible';
        errBox.innerText = resp.message;
    } else {
        location.reload();
    }
}

document.addEventListener("DOMContentLoaded", function() {
    var newNote = document.querySelector("#newNote");
    newNote.addEventListener("submit", addNewNote)

    document.getElementById("newNoteErr").style.visibility = "hidden";
});

