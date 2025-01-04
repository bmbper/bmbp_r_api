use crate::ctx::{base_context, BMBP_TERA};
use salvo::__private::tracing::log::info;
use salvo::prelude::Redirect;
use salvo::prelude::Text::Html;
use salvo::{handler, Depot, Request, Response};
use tera::{Context, Tera};
#[handler]
pub async fn index_view(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    info!("access the root handler, go to workbench page");
    res.render(Redirect::other("/workbench.view"))
}
#[handler]
pub async fn workbench_view(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let  ctx = base_context();
    res.render(Html(
        BMBP_TERA.render("core/work/workbench.html", &ctx).unwrap(),
    ))
}
