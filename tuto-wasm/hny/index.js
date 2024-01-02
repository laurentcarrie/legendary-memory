import {make_double,Universe,plot_component_script,plot_component_script_2 } from "./pkg/hello_wasm.js";



// const counter = document.getElementById("counter");
// const double = document.getElementById("double");
const guess = document.getElementById("guess");
const div_plot = document.getElementById("div_plot");


const renderLoop = (universe) => {
    universe.tick() ;
    var m = universe.the_message() ;
    guess.textContent = m ;

    var draw=true ;

    if (draw) {

        var d = document.createElement('div')
        d.id = 'div_id_' + universe.nb_attempts().toString();
        d.className = "plotly-graph-div";
        d.style = "height:100%; width:100%;";
        document.body.append(d);

        var lastd = document.getElementById('div_plot_' + (universe.nb_attempts() - 1).toString());
        console.log(lastd) ;
        //document.body.removeChild(lastd)


        var z = document.createElement('script');
        z.id = 'the_script_' + universe.nb_attempts().toString();
        console.log(z.id) ;
        z.innerHTML = plot_component_script_2(universe,z.id);
        console.log(z.innerHTML) ;
        document.head.append(z);

        var lastz=document.getElementById("the_script_"+(universe.nb_attempts()-1).toString());
        // document.head.removeChild(lastz) ;
    }



    console.log(universe.done()) ;

    // if (! universe.done()) {
    //
    //     requestAnimationFrame(() => {
    //         setTimeout(renderLoop, 5, universe);
    //     });
    // }


};

export function blahblah() {
    // alert('this is index.js');
    const universe = Universe.new();

    requestAnimationFrame(() => {
         setTimeout(renderLoop, 100,universe);
     });

}


