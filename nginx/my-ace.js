function my_edit(id) {
    console.log("hello") ;
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode("ace/mode/javascript");
}