function my_edit(id,value,mode,nblines) {
    console.log("create edit") ;
    var editor = ace.edit(id);
    console.log("type of editor : ",typeof(editor)) ;
    console.log(JSON.stringify(editor));
    console.log("after stringify") ;
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode(mode);
    // console.log("set value") ;
    // editor.setValue(value);
    // editor.resize() ;
    editor.setOption("maxLines", nblines) ;
    return editor ;
}

function my_set_data(editor,value,nblines) {
    console.log("set value for editor")
    editor.setValue(value);
    editor.setOption("maxLines", nblines) ;
}

function my_set_mode(editor,mode) {
    editor.setOption("mode",mode) ;
}


function my_get_data(editor) {
    // console.log(type_of (editor)) ;
    let data=editor.getValue() ;
    console.log("get data in editor") ;
    // console.log(data) ;
    return data ;
}
