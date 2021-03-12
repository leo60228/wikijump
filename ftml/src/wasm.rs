use lazy_static::lazy_static;
use ouroboros::self_referencing;
use slog::{o, Drain, Logger};
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref LOGGER: Logger = {
        console_log::init().expect("Failed to initialize logging");
        Logger::root(slog_stdlog::StdLog.fuse(), o!())
    };
}

#[wasm_bindgen]
pub fn preprocess(mut source: String) -> String {
    super::preprocess(&*LOGGER, &mut source);
    source
}

#[wasm_bindgen]
#[self_referencing]
pub struct Tokenization {
    buf: String,
    #[borrows(buf)]
    #[covariant]
    inner: crate::Tokenization<'this>,
}

#[wasm_bindgen]
impl Tokenization {
    pub fn tokens(&self) -> Result<JsValue, JsValue> {
        self.with_inner(|inner| {
            let tokens = inner.tokens();
            let js = serde_wasm_bindgen::to_value(&tokens)?;
            Ok(js)
        })
    }
}

#[wasm_bindgen]
pub fn tokenize(source: String) -> Tokenization {
    TokenizationBuilder {
        buf: source,
        inner_builder: |buf: &str| super::tokenize(&*LOGGER, buf),
    }
    .build()
}
