use crate::embed::init_tera;
use std::sync::{Arc, LazyLock};
use tera::{Context, Tera};

pub static BMBP_TERA: LazyLock<Arc<Tera>> = LazyLock::new(|| {
    let tera = init_tera();
    tera.clone()
});

pub fn base_context() -> Context {
    let mut ctx = Context::new();
    ctx.insert("app_title", "AppTitle");
    ctx
}
