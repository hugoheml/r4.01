use serde::{Deserialize, Serialize};

use crate::{domain::VoteOutcome, use_cases::VoteForm};

#[derive(Deserialize)]
pub struct VoteFormV1 {
	pub voter: String,
	pub candidate: String
}

impl From<VoteFormV1> for VoteForm {
	fn from(vote_form: VoteFormV1) -> Self {
		return VoteForm {
			voter: vote_form.voter,
			candidate: vote_form.candidate
		}
	}
}

#[derive(Serialize)]
pub enum VoteOutcomeV1 {
	AcceptedVote(String, String),
	BlankVote(String),
	InvalidVote(String),
	HasAlreadyVoted(String)
}

impl From<VoteOutcome> for VoteOutcomeV1 {
	fn from(vote_outcome: VoteOutcome) -> Self {
		match vote_outcome {
			VoteOutcome::AcceptedVote(voter, candidate) => VoteOutcomeV1::AcceptedVote(voter.0, candidate.0),
			VoteOutcome::BlankVote(voter) => VoteOutcomeV1::BlankVote(voter.0),
			VoteOutcome::InvalidVote(voter) => VoteOutcomeV1::InvalidVote(voter.0),
			VoteOutcome::HasAlreadyVoted(voter) => VoteOutcomeV1::HasAlreadyVoted(voter.0)
		}
	}
}