use js_sys::Math::random;
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::{Plot, Scatter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello xxx , {}!", name));
}

#[wasm_bindgen]
pub fn make_double(i: i32) -> i32 {
    return 2 * i;
}

#[wasm_bindgen]
pub struct Letter {
    guess: u32,
    current: u32,
}

#[wasm_bindgen]
pub struct Universe {
    attempts: Vec<u32>,
    guess: Vec<Letter>,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.guess = self
            .guess
            .iter()
            .map(|l| {
                let guess = l.guess;
                let current = l.current;
                let current = if current == guess {
                    guess
                } else {
                    (random() * 200.0) as u32
                };
                Letter {
                    guess: guess,
                    current: current,
                }
            })
            .collect();

        let success =
            self.guess.iter().fold(
                0u32,
                |acc, l| if l.guess == l.current { acc + 1 } else { acc },
            );

        self.attempts.push(success);
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn nb_attempts(&self) -> u32 {
        let n = self.attempts.len() as u32;
        log(&format!("n = {}", n).to_string());
        n
    }
    pub fn ratio(&self) -> f32 {
        let done: Vec<_> = self.guess.iter().filter(|l| l.guess == l.current).collect();
        (done.len() as f32) / (self.guess.len() as f32)
    }
    pub fn the_message(&self) -> String {
        let current = self
            .guess
            .iter()
            .map(|i| char::from_u32(i.current).unwrap())
            .into_iter()
            .collect();
        current
    }

    pub fn done(&self) -> bool {
        let not_done = self.guess.iter().any(|l| l.guess != l.current);
        !not_done
    }

    pub fn new() -> Universe {
        //let message = "I'm wishing you the best new year & a rich life.\n\nMay this year be filled with prosperity for you!\n\nI wish you have 366 days of never-ending joy in 2024.\n\nCheers to a new year!\n\n-- Laurent --";
        // let message = "blah" ;
        let message = r###"
Happy New Year 2024!
May this year be a remarkable journey of growth and success for you.
As we step into this year, let it be filled with innovative breakthroughs, both in technology and in every aspect of life.
May you find joy in small moments and grand achievements alike.
I hope for health, happiness, and prosperity to be your constant companions.
Let's embrace new challenges with courage and optimism, building stronger connections with those around us.
May this year bring you closer to your dreams and bless you with abundant opportunities for learning and personal development.
Here's to a fantastic 2024!

         -- Laurent -- "###;
        log(message);
        let i = '?' as u32;
        let r = '\n' as u32;
        log_u32(i);
        let guess = message
            .chars()
            .map(|c| Letter {
                current: if c == '\n' { r } else { i },
                guess: c as u32,
            })
            .collect();

        Universe {
            attempts: vec![0],
            guess,
        }
    }
}

#[wasm_bindgen]
pub fn plot_component_script(universe: &Universe, id: String) -> String {
    log(format!("in rust, id = {id}", id = id).as_str());

    let mut plot = Plot::new();

    let x = std::ops::Range {
        start: 0,
        end: universe.attempts.len(),
    }
    .collect();
    let y = &universe.attempts;
    let n = universe.guess.len();
    let nb_found = y.last().unwrap();
    let nb_attempts = y.len();

    let trace = Scatter::new(x, y.to_vec());
    plot.add_trace(trace);
    plot.set_layout(
        plotly::Layout::new()
            .title(Title::new(
                &format!(
                    "Letters found per attempt ( {} found, {} total, {} attempts )",
                    nb_found, n, nb_attempts
                )
                .to_string(),
            ))
            .y_axis(Axis::new().title(Title::new("letters found")))
            .x_axis(Axis::new().title(Title::new("attempts"))),
    );
    // plot.use_local_plotly();
    // plot.show()

    let s = plot.to_inline_html(Some(&id));
    let mut s: Vec<&str> = s.split("\n").collect();
    // log_u32(s.len().try_into().unwrap()) ;
    s.remove(0);
    // log_u32(s.len().try_into().unwrap()) ;
    s.remove(0);
    // log_u32(s.len().try_into().unwrap()) ;
    s.pop();
    // log_u32(s.len().try_into().unwrap()) ;
    let ss = s.iter().fold("".to_string(), |acc, s| acc + "\n" + s);
    // log(&ss) ;
    ss
}

//
// #[wasm_bindgen]
// pub fn plot_component_script(universe:& Universe) -> String {
//
//     let x = std::ops::Range{start:0,end:universe.attempts.len()}.fold(String::from(""),|acc,i| format!("{acc},{i}",acc=acc,i=i).to_string().clone()) ;
//     let y = universe.attempts.iter().fold(String::from(""),|acc,i| format!("{acc},{i}",acc=acc,i=i).to_string().clone()) ;
//
//
//
//     let s = format!(r###"
//         Plotly.newPlot("script_id_{n}", {{
//         "data": [
//     {{
//         "type": "scatter",
//         "x": [ {x} ],
//         "y": [ {y} ]
//     }}
//         ],
//         "layout": {{}},
//         "config": {{}}
//     }})
//     ;
//     "###,n=universe.attempts.len(),x=x,y=y);
//     s.to_string()
// }
