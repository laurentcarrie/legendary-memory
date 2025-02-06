import {get_data} from "./song_lsdir.js";

const id1 = document.getElementById("id1");



export function blahblah() {
    // alert('this is index.js');
    var data = get_data() ;
    console.log(data) ;

    id1.innerHTML = data;

}
