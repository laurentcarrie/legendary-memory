function my_edit(id,value,mode,nblines) {
    console.log("create edit") ;
    var editor = ace.edit(id);
    console.log("type of editor : ",typeof(editor)) ;
    // var j = JSON.string
    // console.log(JSON.stringify(editor));
    // console.log("after stringify") ;
    editor.setTheme("ace/theme/twilight");
    editor.session.setMode(mode);
    // console.log("set value") ;
    // editor.setValue(value);
    // editor.resize() ;
    editor.setOption("maxLines", nblines) ;
    return editor ;
}

function my_set_data(id,value,nblines) {
    var editor = ace.edit(id);
    console.log("set value for editor")
    editor.setValue(value);
    editor.setOption("maxLines", nblines) ;
}

function my_set_mode(id,mode) {
    var editor = ace.edit(id);
    editor.setOption("mode",mode) ;
}


function my_get_data(id) {
    // console.log(type_of (editor)) ;
    var editor = ace.edit(id);
    let data= editor.getValue() ;
    console.log("get data in editor") ;
    console.log(data) ;
    return data ;
}
