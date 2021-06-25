use actix_web::web;
mod path;
mod auth;
mod app;
mod users;
mod to_do;

pub mod token;
pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
    app::app_factory(app);
    users::user_factory(app);
}