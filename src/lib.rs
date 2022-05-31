mod utils;

use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, ImageData, Window,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct AverageColor {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl AverageColor {
    pub fn get_color_from_image(&self, resource: HtmlImageElement, algo: &str) -> Array {
        let canvas = self.canvas.clone();
        let ctx = self.ctx.clone();
        let height = resource.height() as f64;
        let width = resource.width() as f64;

        canvas.set_height(height as u32);
        canvas.set_width(width as u32);

        ctx.draw_image_with_html_image_element(&resource, 0.0, 0.0)
            .expect("failed to draw image to <canvas> element");

        let image_data: ImageData = ctx.get_image_data(0.0, 0.0, height, width).unwrap();
        let pixels = image_data.data().to_vec();
        let len = pixels.len();

        match algo {
            "simple" => simp(pixels, len, 4)
                .into_iter()
                .map(JsValue::from)
                .collect(),
            "sqrt" => sqrt_algo(pixels, len, 4)
                .into_iter()
                .map(JsValue::from)
                .collect(),
            _ => vec!["empty"].into_iter().map(JsValue::from).collect(),
        }
    }

    // TODO
    //pub fn get_color_from_canvas(&self, resource: HtmlCanvasElement) {}
    //pub fn get_color_from_video(&self, resource: HtmlVideoElement) {}

    fn get_window() -> Window {
        web_sys::window().unwrap()
    }

    fn get_document() -> Document {
        AverageColor::get_window().document().unwrap()
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> AverageColor {
        let canvas = AverageColor::get_document()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .expect("failed to create buffer <canvas> element");

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("failed to obtain 2d rendering context for buffer <canvas>");

        AverageColor { canvas, ctx }
    }
}

fn simp(pixels: Vec<u8>, length: usize, step: usize) -> Vec<String> {
    let mut r: u64 = 0;
    let mut g: u64 = 0;
    let mut b: u64 = 0;
    let mut a: u64 = 0;
    let count: f32 = (length / step) as f32;

    for i in (0..length).step_by(step) {
        let alpha = pixels[i + 3] as u64;
        r += pixels[i] as u64 * alpha;
        g += pixels[i + 1] as u64 * alpha;
        b += pixels[i + 2] as u64 * alpha;
        a += alpha;
    }

    console_log!("{}", r);

    let r = (r / a) as u8;
    let g = (g / a) as u8;
    let b = (b / a) as u8;
    let a = a as f32 / count;

    vec![get_rgb(r, g, b), get_rgba(r, g, b, a), get_hex(r, g, b)]
}

fn sqrt_algo(pixels: Vec<u8>, length: usize, step: usize) -> Vec<String> {
    let mut r: u64 = 0;
    let mut g: u64 = 0;
    let mut b: u64 = 0;
    let mut a: u64 = 0;
    let count: f32 = (length / step) as f32;

    for i in (0..length).step_by(step) {
        let red = pixels[i] as u64;
        let green = pixels[i + 1] as u64;
        let blue = pixels[i + 2] as u64;
        let alpha = pixels[i + 3] as u64;

        r += (red * red) * alpha;
        g += (green * green) * alpha;
        b += (blue * blue) * alpha;
        a += alpha;
    }

    let r = ((r / a) as f64).sqrt() as u8;
    let g = ((g / a) as f64).sqrt() as u8;
    let b = ((b / a) as f64).sqrt() as u8;
    let a = a as f32 / count;

    vec![get_rgb(r, g, b), get_rgba(r, g, b, a), get_hex(r, g, b)]
}

fn get_rgb(r: u8, g: u8, b: u8) -> String {
    format!("rgb({}, {}, {})", r, g, b)
}

fn get_rgba(r: u8, g: u8, b: u8, a: f32) -> String {
    format!("rgba({}, {}, {}, {})", r, g, b, a)
}

fn get_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}
