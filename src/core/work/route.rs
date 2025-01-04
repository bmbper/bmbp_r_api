use crate::core::work::action::{index_action};
use salvo::{ Router};
use crate::core::work::view::{index_view, workbench_view};

pub fn build_work_route() -> Router {
    let mut router = Router::new();
    router = router.get(index_view).post(index_action);
    router = router.push(Router::with_path("/workbench.view").get(workbench_view));
    router
}
