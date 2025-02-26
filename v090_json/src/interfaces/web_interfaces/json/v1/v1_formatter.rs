use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{domain::{Scoreboard, VoteOutcome, VotingMachine}, use_cases::VoteForm};

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

#[derive(Serialize)]
struct ScoreboardV1 {
	scores: BTreeMap<String, usize>,
	blank_score: usize,
	invalid_score: usize
}

impl From<Scoreboard> for ScoreboardV1 {
	fn from(scoreboard: Scoreboard) -> Self {
		Self {
			scores: scoreboard.scores.iter().map(|(k, v)| (k.to_string(), v.0)).collect(),
			blank_score: scoreboard.blank_score.0,
			invalid_score: scoreboard.invalid_score.0
		}
	}
}

#[derive(Serialize)]
pub struct VotingMachineV1 {
	voters: BTreeSet<String>,
	scoreboard: ScoreboardV1
}

impl From<VotingMachine> for VotingMachineV1 {
    fn from(machine: VotingMachine) -> Self {
        Self {
            voters: machine.get_voters().0.iter().map(|v| v.to_string()).collect(),
            scoreboard: ScoreboardV1::from(machine.get_scoreboard().clone()),
        }
    }
}