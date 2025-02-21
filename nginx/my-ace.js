function my_edit(id,value,mode,nblines) {
    console.log("create edit") ;
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode(mode);
    editor.setOption("maxLines", nblines) ;
    return editor ;
}

function my_set_data(id,value,mode,nblines) {
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode(mode);
    editor.setValue(value);
    editor.setOption("maxLines", nblines) ;
}

function my_set_mode(id,mode) {
    var editor = ace.edit(id);
    editor.setOption("mode",mode) ;
}


function my_get_data(id) {
    var editor = ace.edit(id);
    let data= editor.getValue() ;
    return data ;
}

function my_commit_message() {
    let text = prompt("enter a commit message:", "no message");
    return text ;
}