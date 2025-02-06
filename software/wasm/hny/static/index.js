import {make_double, Universe, plot_component_script} from "./dsci_hny_wasm_hny.js";

const guess = document.getElementById("guess");
const div_plot = document.getElementById("div_plot");


const renderLoop = (universe) => {
    universe.tick();
    var m = universe.the_message();
    guess.textContent = m;

    var draw = true;

    if (draw) {

        var d = document.createElement('div')
        d.id = 'div_id_' + universe.nb_attempts().toString();
        d.className = "plotly-graph-div";
        d.style = "height:80%; width:80%;";
        document.body.append(d);

        var lastd = document.getElementById('div_id_' + (universe.nb_attempts() - 1).toString());
        // console.log(lastd) ;
        document.body.removeChild(lastd)


        var z = document.createElement('script');
        var id = 'div_id_' + universe.nb_attempts().toString();
        z.id = "script_" + id
        // console.log(z.id) ;
        z.innerHTML = plot_component_script(universe, id);
        // console.log(z.innerHTML) ;
        document.head.append(z);

        var lastz = document.getElementById("script_div_id_" + (universe.nb_attempts() - 1).toString());
        document.head.removeChild(lastz);
    }

    var text_id = document.getElementById("hny-text");
    // const randomColor = Math.floor(Math.random()*16777215).toString(16);
    var ratio = universe.ratio();
    var antiratio = 1 - ratio;

    const redmin = 30;
    const redmax = 200;

    const greenmin = 255;
    const greenmax = 100;


    var red = Math.floor(ratio * redmax + antiratio * redmin);
    var green = Math.floor(ratio * greenmax + antiratio * greenmin);
    var blue = Math.floor(ratio * 255);
    // var color = 16*16*16*16*red + 16*16*green + blue ;
    // const randomColor = Math.floor(ratio*16777215).toString(16);
    // const randomColor = Math.floor(color).toString(16);
    var color = "#" + (red).toString(16) + (green).toString(16) + (blue).toString(16);
    console.log(color);

    text_id.style.backgroundColor = color;

    console.log(universe.done());

    if (!universe.done()) {
        requestAnimationFrame(() => {
            setTimeout(renderLoop, 5, universe);
        });
    } else {
        text_id.style.backgroundColor = "lavender";
    }


};

export function blahblah() {
    // alert('this is index.js');
    const universe = Universe.new();

    requestAnimationFrame(() => {
        setTimeout(renderLoop, 100, universe);
    });

}
