use salvo::Router;

mod auth;
mod work;
mod setting;
mod rbac;
mod abc;
mod route;

pub use route::build_core_route;