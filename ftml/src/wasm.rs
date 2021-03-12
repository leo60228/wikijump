use lazy_static::lazy_static;
use slog::{o, Drain, Logger};
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref LOGGER: Logger = {
        console_log::init().expect("Failed to initialize logging");
        Logger::root(slog_stdlog::StdLog.fuse(), o!())
    };
}

#[wasm_bindgen]
pub fn preprocess(source: &str) -> String {
    let mut string = source.into();
    super::preprocess(&*LOGGER, &mut string);
    string
}

#[wasm_bindgen]
pub fn tokenize(source: &str) -> Result<JsValue, JsValue> {
    let tokenization = super::tokenize(&*LOGGER, source);
    let tokens = tokenization.tokens();
    let serialized = serde_wasm_bindgen::to_value(&tokens)?;
    Ok(serialized)
}
