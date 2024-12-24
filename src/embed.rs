use std::sync::Arc;
use rust_embed::RustEmbed;
use salvo::Router;
use salvo::serve_static::static_embed;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "web/static"]
struct StaticAssets;

#[derive(RustEmbed)]
#[folder = "web/templates"]
pub(crate) struct PageAssets;
fn init_tera() -> Arc<Tera> {
    let tera = Tera::new("web/templates/**/*").unwrap_or_else(|e| {
        panic!("Failed to initialize Tera templates: {}", e);
    });
    Arc::new(tera)
}
pub(crate) fn build_template_router(mut router: Router) -> Router {
    router
}
pub fn build_static_router(mut router: Router) -> Router {
    router = router.push(Router::with_path("/static/<**path>").get(static_embed::<StaticAssets>()));
    router
}

