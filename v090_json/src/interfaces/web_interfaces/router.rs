use axum::{routing::post, Router};
use axum::routing::get;

use crate::storage::Storage;

use super::json;
use super::{html::html_handlers, web_routers::WebRoutes, AxumState};

pub fn make_router<Store: Storage + Clone + Send + Sync + 'static>(
	app_state: AxumState<Store>,
	routes: &WebRoutes
) -> Router {
	let v1_routes = Router::new()
		.route(routes.results, get(json::v1::v1_handlers::get_results))
		.route(routes.vote, post(json::v1::v1_handlers::vote))
		.with_state(app_state.clone());

	let json_routes = Router::new().nest(routes.v1, v1_routes);

	Router::new()
		.route(routes.index, get(html_handlers::get_index))
		.route(routes.vote, post(html_handlers::vote))
		.route(routes.results, get(html_handlers::get_results))
		.nest(routes.json, json_routes)
		.with_state(app_state)

}