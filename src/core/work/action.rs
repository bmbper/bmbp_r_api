use salvo::prelude::Redirect;
use salvo::{handler, Depot, Request, Response};
use tracing_log::log::info;


#[handler]
pub async fn index_action(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    res.render("Welcome to Bmbp V2 API Service.");
}

