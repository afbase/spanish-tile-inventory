use log::info;
use log::LevelFilter;
use yew::prelude::*;
use chrono::{DateTime, Utc};
use log::{Level, Metadata, Record};
use app::App;
extern crate console_error_panic_hook;
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

fn main() {
    console_error_panic_hook::set_once();
    // wasm_logger::init(wasm_logger::Config::default());
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
    info!("{}", "start!");
    web_sys::window()
        .and_then(|window| window.document())
        .map_or_else(
            || {
                let log_str = "failed to load wasm module successfully part 1";
                let log_string = String::from(log_str);
                info!("{}", log_string);
                panic!("{}", log_str);
            },
            |document| match document.get_element_by_id(DIV_BLOG_NAME) {
                Some(div_element) => {
                    info!("{}", "here we go!");
                    let renderer = yew::Renderer::<App>::with_root(div_element);
                    info!("{}", "rendering");
                    renderer.render();
                    info!("{}", "rendered");
                }
                None => {
                    let log_str = format!(
                        "Unable to find div {DIV_BLOG_NAME}. failed to load wasm module successfully part 2"
                    );
                    info!("{}", log_str);
                    panic!("{}", log_str);
                }
            },
        );
    // log::set_logger(&MY_LOGGER).unwrap();
    // log::set_max_level(LevelFilter::Info);
    // info!("{}", "start!");
    // web_sys::window()
    //     .and_then(|window| window.document())
    //     .and_then(|document| document.get_element_by_id(DIV_BLOG_NAME))
    //     .map_or_else(
    //         || {
    //             let log_str = format!(
    //                 "Unable to find div {DIV_BLOG_NAME}. Failed to load wasm module successfully."
    //             );
    //             info!("{}", log_str);
    //             panic!("{}", log_str);
    //         },
    //         |element| {
    //             
    //             let div_element = element.dyn_into::<HtmlElement>().unwrap();
    //             info!("{}", "unwrapped");
    //             let renderer = yew::Renderer::<App>::with_root(div_element.into());
    //             info!("{}", "rendering");
    //             renderer.render();
    //         },
    //     );
}
