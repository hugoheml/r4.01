use anyhow::Result;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{interfaces::web_interfaces::{AxumError, AxumState}, storage::Storage};

use super::v1_formatter::{VoteFormV1, VoteOutcomeV1, VotingMachineV1};

pub async fn vote<Store: Storage>(
	State(mut app_state): State<AxumState<Store>>,
	Json(vote_form): Json<VoteFormV1>
 ) -> Json<VoteOutcomeV1> {

	let vote_form = vote_form.into();
	let vote_outcome = app_state.controller.vote(vote_form).await;

	match vote_outcome {
		Ok(outcome) => Json(outcome.into()),
		Err(e) => Json(VoteOutcomeV1::InvalidVote(e.to_string()))
	}
}

pub async fn get_results<Store: Storage>(
	State(app_state): State<AxumState<Store>>
) -> Result<impl IntoResponse, AxumError> {
	let voting_machine = app_state.controller.get_voting_machine().await?;

	Ok((StatusCode::OK, Json(VotingMachineV1::from(voting_machine))).into_response())
}