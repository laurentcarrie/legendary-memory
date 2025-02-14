function my_edit(id,value) {
    console.log("create edit") ;
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode("ace/mode/json");
    console.log("set value") ;
    editor.setValue(value);
    editor.resize() ;
    editor.setOption("maxLines", 1000).
    return editor ;
}

function my_set_data(editor,data) {
    console.log("set data in editor") ;
    console.log(editor) ;
    console.log(data) ;
    editor.setValue(data) ;
}