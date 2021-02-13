
function delete_note(id) {
    var xhttp = new XMLHttpRequest();
    xhttp.open('DELETE', '/notes/' + id, true);
    xhttp.send();
}
