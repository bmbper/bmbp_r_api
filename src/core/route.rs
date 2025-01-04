use crate::core::auth::build_auth_route;
use crate::core::rbac::build_rbac_route;
use crate::core::setting::build_setting_route;
use crate::core::work::build_work_route;
use salvo::Router;

pub fn build_core_route(mut router: Router) -> Router {
    router = router
        .push(build_auth_route())
        .push(build_work_route())
        .push(build_setting_route())
        .push(build_rbac_route());
    router
}
