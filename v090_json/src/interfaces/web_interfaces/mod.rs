mod html;
mod json;
pub mod router;
pub mod web_routers;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;
use web_routers::WebRoutes;

use crate::use_cases::VotingController;

use super::lexicon::Lexicon;

#[derive(Error, Debug)]
#[error("Error: {0}")]
pub struct AxumError(# [from] anyhow::Error);

impl IntoResponse for AxumError {
	fn into_response(self) -> Response {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong: {}", self),
		)
		.into_response()
	}
}

#[derive(Clone)]
pub struct AxumState<Store> {
	pub controller: VotingController<Store>,
	pub routes: WebRoutes,
	pub lexicon: Lexicon
}