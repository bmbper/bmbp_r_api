use crate::core::work::action::{index_action};
use salvo::{ Router};

pub fn build_work_route() -> Router {
    let mut router = Router::new();
    router = router.get(index_action).post(index_action);
    router
}
