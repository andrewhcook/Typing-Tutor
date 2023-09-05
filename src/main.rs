use std::fs::read_to_string;
use serde::{Serialize, Deserialize};
use serde_json;
use nannou::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use nannou::event::{ElementState, KeyboardInput};
use nannou::winit::event::VirtualKeyCode;
fn main() {
 
        nannou::app(model)
        .update(update)
        .simple_window(view)
        .event(event)
        .run();
}

struct LiveWord {
    x: f64,
    y: f64,
    word: String,
    velocity: f64,
}

struct Model {
    threes: Vec<String>,
    fours: Vec<String>,
    fives: Vec<String>,
    sixes: Vec<String>,
    sevens: Vec<String>,
    longer_than_sevens: Vec<String>,
    live_words: Vec<LiveWord>,
    score: u64,
    level: u8,
    words_per_minute: u16,
    counter: f32,
    guess_word: String
}

fn model<'a>(app: &App) -> Model {
    let dictionary_info = std::fs::read_to_string("./src/dictionary_compact.json").expect("Error Reading Dictionary File");
    let parsed_dictionary: serde_json::Value = serde_json::from_str(&dictionary_info).expect("Error parsing into json");
    let mut keys = vec![];
    if let Some(map) = parsed_dictionary.as_object() {
        keys = map.keys().collect();
    }

    let three_letter_words = keys.clone().into_iter().filter(|item| {item.len() == 3 as usize}).cloned().collect::<Vec<String>>();
    let four_letter_words = keys.clone().into_iter().filter(|item| {item.len() == 4 as usize}).cloned().collect::<Vec<String>>();
    let five_letter_words = keys.clone().into_iter().filter(|item| {item.len() == 5 as usize}).cloned().collect::<Vec<String>>();
    let six_letter_words = keys.clone().into_iter().filter(|item| {item.len() == 6 as usize}).cloned().collect::<Vec<String>>();
    let seven_letter_words = keys.clone().into_iter().filter(|item| {item.len() == 7 as usize}).cloned().collect::<Vec<String>>();
    let longer_than_seven_letter_words = keys.clone().into_iter().filter(|item| {item.len() > 7 as usize}).cloned().collect::<Vec<String>>();


    Model {threes: three_letter_words.clone(), fours: four_letter_words.clone(), fives: five_letter_words.clone(), 
        sixes: six_letter_words.clone(), sevens: seven_letter_words.clone(), longer_than_sevens: longer_than_seven_letter_words.clone(),
         live_words: vec![], score: 0, level: 1, words_per_minute: 25, counter: 0.0, guess_word:"".to_owned()}
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(event),
        } => match event {
            WindowEvent::KeyReleased(key) => {
                match key {
                    VirtualKeyCode::Return => {
                        for i in &mut model.live_words {
                            if i.word == model.guess_word {
                                model.score +=1;

                            }
                        }
                        let new_words: Vec<LiveWord> = model.live_words
                        .iter()
                        .filter(|word| word.word != model.guess_word)
                        .map(|word| LiveWord {
                            x: word.x,
                            y: word.y,
                            word: word.word.clone(),
                            velocity: word.velocity
                        })
                        .collect();
                    model.live_words = new_words;
                    model.guess_word = "".to_owned();
                    }, 
                _ => {
                    let new_word = key_to_str(key);
                    match new_word {
                        Some(word) => {model.guess_word.push_str(word)},
                        _ => {}
                    }}
                }
                
                    
                
            }
            _ => {}
        },
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, update: Update){
    model.counter +=  (model.words_per_minute as f32 + (model.score / 10) as f32) / 60.0;
    let coeff =  model.level;
    if model.counter >= 360.0 {
        model.counter = 0.0;
    };

    
        for  i in &mut model.live_words {
            let step = coeff as f64 *  i.velocity / 60.0;
            i.y -= step;
            if i.y < app.window_rect().bottom() as f64 {
                i.y = app.window_rect().top() as f64;
            }
        }

        
     if model.counter > 60.0  {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(3..=8);
        let word_bank = match num {
            3 => {&model.threes},
            4 => {&model.fours},
            5 => {&model.fives},
            6 => {&model.sixes},
            7 => {&model.sevens},
            8 => {&model.longer_than_sevens}
            _ => {unreachable!()}

        };
        let word_in_new_word = word_bank.choose(&mut rand::thread_rng()).unwrap().clone();
        let x = rand::thread_rng ().gen_range (app.window_rect().left()as i16+20..app.window_rect().right() as i16 -20 ) as f64;
        let y = app.window_rect().top() as f64;
        let velocity = rand::thread_rng ().gen_range (1..25) as f64;
        let word = LiveWord {
            x: x,
            y: y,
            word: word_in_new_word,
            velocity: velocity,
        };
        model.live_words.push(word);
        model.counter = 0.0;
    }
    
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.text(&model.score.to_string()).x_y(0.0, 0.0).font_size(30).color(GREEN);
    for i in &model.live_words {
        draw.text(&i.word).x_y(i.x as f32, i.y as f32).color(BLACK).font_size(18);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn key_to_str(key: Key) -> Option<&'static str> {
    match key {
        Key::A => Some("a"),
        Key::B => Some("b"),
        Key::C => Some("c"),
        Key::D => Some("d"),
        Key::E => Some("e"),
        Key::F => Some("f"),
        Key::G => Some("g"),
        Key::H => Some("h"),
        Key::I => Some("i"),
        Key::J => Some("j"),
        Key::K => Some("k"),
        Key::L => Some("l"),
        Key::M => Some("m"),
        Key::N => Some("n"),
        Key::O => Some("o"),
        Key::P => Some("p"),
        Key::Q => Some("q"),
        Key::R => Some("r"),
        Key::S => Some("s"),
        Key::T => Some("t"),
        Key::U => Some("u"),
        Key::V => Some("v"),
        Key::W => Some("w"),
        Key::X => Some("x"),
        Key::Y => Some("y"),
        Key::Z => Some("z"),
        Key::Minus => {Some("-")},
        Key::Underline => {Some("_")},
        Key::Space => {Some(" ")},
        _ => None,
    }
}