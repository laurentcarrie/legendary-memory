// import {mymy_greet} from "/leptos/source_tree/source_tree.js";

function xxx() {
    wasmBindings.mymy_greet() ;
}

// function my_edit(id,value,mode,nblines) {
//     console.log("create edit") ;
//     var editor = ace.edit(id);
//     // editor.setTheme("ace/theme/twilight");
//     editor.setTheme("ace/theme/github");
//     editor.session.setMode(mode);
//     editor.setOption("maxLines", nblines) ;
//     editor.setOption("autoScrollEditorIntoView",true) ;
//     // editor.getSession().on('change', xxx) ;
//     return editor ;
// }


function my_set_data(id,value,mode,nblines) {
    console.log("my set data") ;
    var editor = ace.edit(id);
    // editor.setTheme("ace/theme/twilight");
    editor.setTheme("ace/theme/github");
    editor.session.setMode(mode);
    editor.session.off('change', wasmBindings.on_change_editor);
    editor.setValue(value);
    editor.setOption("maxLines", nblines) ;
    editor.setOption("autoScrollEditorIntoView",true) ;
    editor.session.on('change', wasmBindings.on_change_editor) ;
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
