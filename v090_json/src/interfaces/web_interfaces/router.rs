use axum::{routing::post, Router};
use axum::routing::get;

use crate::storage::Storage;

use super::{html::{self, html_handlers}, web_routers::WebRoutes, AxumState};

pub fn make_router<Store: Storage + Clone + Send + Sync + 'static>(
	app_state: AxumState<Store>,
	routes: &WebRoutes
) -> Router {
	Router::new()
		.route(routes.index, get(html_handlers::get_index))
		.route(routes.vote, post(html_handlers::vote))
		.route(routes.results, get(html_handlers::get_results))
		.with_state(app_state)

}