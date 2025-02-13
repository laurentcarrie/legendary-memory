function my_edit(id) {
    console.log("create edit") ;
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode("ace/mode/javascript");
    console.log("set value") ;
    editor.setValue("the new text here");
    return editor ;
}

function my_set_data(editor,data) {
    console.log("set data in editor") ;
    editor.setValue(data) ;
}