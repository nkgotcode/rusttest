use ebur128::*;
use std::io::{stdout};
use wasm_bindgen::prelude::*;
// fn main() {
//     println!("Hello, world!");
//     let a = ebur128::EbuR128::new(2,44100,ebur128::Mode::I);
//     println!("Generated: {:?}", a);
// }
// class AudioProcessor {
    
// }
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn processAudio(audio_buffer: *const f32, buffer_length: usize) {
    alert(&format!("Hello, Web Assembly!"));
    let samples = unsafe {std::slice::from_raw_parts(audio_buffer, buffer_length)};
    let a = ebur128::EbuR128::new(2,44100,ebur128::Mode::I);
    println!("Generated: {:?}", a);
}