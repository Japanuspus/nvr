function invoke(arg) {window.external.invoke(JSON.stringify(arg));};
function update_note_list(note_names) {
    var ul = document.createDocumentFragment();
    note_names.forEach(note_name => {
        var li = document.createElement('li');
        li.textContent = note_name;
        li.tabIndex = -1;
        ul.appendChild(li);
    });
    var container = document.getElementById("note_list");
    container.innerHTML='';
    container.appendChild(ul);
};
invoke({"type": "update"});
