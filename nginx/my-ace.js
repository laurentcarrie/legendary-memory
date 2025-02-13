function my_edit(id) {
    console.log("hello") ;
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode("ace/mode/javascript");
    console.log("set value") ;
    editor.setValue("the new text here");
    return editor ;
}