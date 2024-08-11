use log::info;
use log::LevelFilter;
use web_sys::{HtmlElement, wasm_bindgen::JsCast};
use yew::prelude::*;
use chrono::{DateTime, Utc};
use log::{Level, Metadata, Record};
pub struct MyLogger;
pub static MY_LOGGER: MyLogger = MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    #[cfg(not(target_family = "wasm"))]
    fn log(&self, record: &Record) {
        let now: DateTime<Utc> = Utc::now();
        if self.enabled(record.metadata()) {
            println!(
                "[{}] {} - {}",
                now.to_rfc3339(),
                record.level(),
                record.args()
            );
        }
    }

    #[cfg(target_family = "wasm")]
    fn log(&self, record: &Record) {
        use gloo_console::log as gloo_log;
        use js_sys::JsString;
        let now: DateTime<Utc> = Utc::now();
        if self.enabled(record.metadata()) {
            let str_log: JsString = format!(
                "[{}] {} - {}",
                now.to_rfc3339(),
                record.level(),
                record.args()
            )
            .into();
            gloo_log!(str_log);
        }
    }

    fn flush(&self) {}
}

const DIV_BLOG_NAME: &str = "spanish-tiles-nola";

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Your existing view implementation goes here
        // This should already include your survey content
        html! {
            // Your existing survey content
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(DIV_BLOG_NAME))
        .map_or_else(
            || {
                let log_str = format!(
                    "Unable to find div {DIV_BLOG_NAME}. Failed to load wasm module successfully."
                );
                info!("{}", log_str);
                panic!("{}", log_str);
            },
            |element| {
                let div_element = element.dyn_into::<HtmlElement>().unwrap();
                let renderer = yew::Renderer::<App>::with_root(div_element.into());
                renderer.render();
            },
        );
}
