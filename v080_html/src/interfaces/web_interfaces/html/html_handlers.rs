use axum::{extract::State, response::IntoResponse, Form};

use crate::{interfaces::web_interfaces::{web_routers::WEB_ROUTES, AxumError, AxumState}, storage::Storage, use_cases::VoteForm};

use super::html_formatter;

pub async fn get_index<Store: Storage>(State(app_state): State<AxumState<Store>>) -> Result<impl IntoResponse, AxumError> {
	let lexicon = app_state.lexicon;
	let voting_machine = app_state.controller.get_voting_machine().await?;

	Ok(
		html_formatter::index(&WEB_ROUTES, &lexicon, &voting_machine)
	)
}

pub async fn get_results<Store: Storage>(State(app_state): State<AxumState<Store>>) -> Result<impl IntoResponse, AxumError> {
	let lexicon = app_state.lexicon;
	let voting_machine = app_state.controller.get_voting_machine().await?;
	
	Ok(
		html_formatter::voting_machine(&WEB_ROUTES, &lexicon, &voting_machine)
	)
}

pub async fn vote<Store: Storage>(State(app_state): State<AxumState<Store>>, Form(vote_form): Form<VoteForm>) -> Result<impl IntoResponse, AxumError> {
	let lexicon = app_state.lexicon;
	
	Ok(
		html_formatter::vote_form(&WEB_ROUTES, &lexicon)
	)

}