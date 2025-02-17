function my_edit(id,value,nblines) {
    console.log("create edit") ;
    var editor = ace.edit(id);
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode("ace/mode/json");
    // console.log("set value") ;
    // editor.setValue(value);
    // editor.resize() ;
    // editor.setOption("maxLines", nblines) ;
    return editor ;
}

function my_set_data(editor,value,nblines) {
    console.log("set value for editor")
    editor.setValue(value);
    editor.setOption("maxLines", nblines) ;
}


function my_get_data(editor) {
    let data=editor.getValue() ;
    console.log("get data in editor") ;
    // console.log(data) ;
    return data ;
}