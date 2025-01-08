use crate::core::setting::{dict, vars};
use salvo::Router;

pub fn build_setting_route() -> Router {
    let mut router = Router::with_path("/setting");
    router = router
        .push(
            Router::with_path("/dict")
                .push(Router::with_path("/tree").post(dict::handler::tree))
                .push(Router::with_path("/tree/ignore/node").post(dict::handler::tree_ignore_node))
                .push(Router::with_path("/page").post(dict::handler::page))
                .push(Router::with_path("/list").post(dict::handler::list))
                .push(Router::with_path("/info").post(dict::handler::info))
                .push(Router::with_path("/save").post(dict::handler::save))
                .push(Router::with_path("/enable").post(dict::handler::enable))
                .push(Router::with_path("/disable").post(dict::handler::disable))
                .push(Router::with_path("/delete").post(dict::handler::delete))
                .push(Router::with_path("/batch/enable").post(dict::handler::batch_enable))
                .push(Router::with_path("/batch/disable").post(dict::handler::batch_disable))
                .push(Router::with_path("/batch/delete").post(dict::handler::batch_delete))
                .push(Router::with_path("/update/parent").post(dict::handler::update_parent))
                .push(Router::with_path("/combo").post(dict::handler::combo))
                .push(Router::with_path("/combos").post(dict::handler::combos))
                .push(Router::with_path("/combo/tree").post(dict::handler::combo_tree))
                .push(Router::with_path("/combos/tree").post(dict::handler::combos_tree))
                .push(Router::with_path("/display").post(dict::handler::display))
                .push(Router::with_path("/displays").post(dict::handler::displays))
                .push(Router::with_path("/display/tree").post(dict::handler::display_tree))
                .push(Router::with_path("/displays/tree").post(dict::handler::displays_tree)),
        )
        .push(
            Router::with_path("/vars")
                .push(Router::with_path("/tree").post(vars::handler::tree))
                .push(Router::with_path("/tree/ignore/node").post(vars::handler::tree_ignore_node))
                .push(Router::with_path("/page").post(vars::handler::page))
                .push(Router::with_path("/list").post(vars::handler::list))
                .push(Router::with_path("/info").post(vars::handler::info))
                .push(Router::with_path("/save").post(vars::handler::save))
                .push(Router::with_path("/enable").post(vars::handler::enable))
                .push(Router::with_path("/disable").post(vars::handler::disable))
                .push(Router::with_path("/delete").post(vars::handler::delete))
                .push(Router::with_path("/batch/enable").post(vars::handler::batch_enable))
                .push(Router::with_path("/batch/disable").post(vars::handler::batch_disable))
                .push(Router::with_path("/batch/delete").post(vars::handler::batch_delete))
                .push(Router::with_path("/update/parent").post(vars::handler::update_parent)),
        );
    router
}
