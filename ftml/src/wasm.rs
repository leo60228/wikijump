use crate::parsing::Token;
use lazy_static::lazy_static;
use ouroboros::self_referencing;
use slog::{o, Drain, Logger};
use std::sync::Arc;
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

#[self_referencing]
#[derive(Debug)]
struct TokenizationInner {
    buf: String,
    #[borrows(buf)]
    #[covariant]
    inner: crate::Tokenization<'this>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Tokenization(Arc<TokenizationInner>);

#[wasm_bindgen]
impl Tokenization {
    pub fn tokens(&self) -> Result<JsValue, JsValue> {
        self.0.with_inner(|inner| {
            let tokens = inner.tokens();
            let js = serde_wasm_bindgen::to_value(&tokens)?;
            Ok(js)
        })
    }
}

#[wasm_bindgen]
pub fn tokenize(source: String) -> Tokenization {
    Tokenization(Arc::new(
        TokenizationInnerBuilder {
            buf: source,
            inner_builder: |buf: &str| super::tokenize(&*LOGGER, buf),
        }
        .build(),
    ))
}

#[wasm_bindgen]
#[self_referencing]
#[derive(Debug)]
pub struct Parser {
    tokenization: Arc<TokenizationInner>,
    #[borrows(tokenization)]
    #[covariant]
    inner: crate::parsing::Parser<'this, 'this>,
}

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn parse(tokenization: Tokenization) -> Self {
        ParserBuilder {
            tokenization: tokenization.0,
            inner_builder: |tokenization: &TokenizationInner| {
                crate::parsing::Parser::new(&*LOGGER, tokenization.borrow_inner())
            },
        }
        .build()
    }

    pub fn consume(&mut self) -> Result<Vec<JsValue>, JsValue> {
        let parse_result =
            self.with_inner_mut(|inner| crate::parsing::consume(&*LOGGER, inner));
        let elements = parse_result.unwrap().item; // TODO: handle errors/warnings

        let values = elements
            .into_iter()
            .map(|elem| serde_wasm_bindgen::to_value(&elem))
            .collect::<Result<_, _>>()?;

        Ok(values)
    }

    pub fn eof(&self) -> bool {
        self.borrow_inner().current().token == Token::InputEnd
    }
}
