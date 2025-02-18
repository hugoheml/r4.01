use serde::Deserialize;

use crate::{domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}, storage::Storage};

#[derive(Deserialize)]
pub struct VoteForm {
	pub voter: String,
	pub candidate: String
}

impl From<VoteForm> for BallotPaper {
	fn from(form: VoteForm) -> Self {
		BallotPaper {
			voter: Voter(form.voter),
			candidate: if form.candidate.is_empty() { None } else { Some(Candidate(form.candidate)) }
		}
	}
}

pub struct VotingController<Store> {
	store: Store
}

impl<Store: Storage> VotingController<Store> {
	pub fn new(store: Store) -> Self {
		Self {
			store
		}
	}

	pub async fn vote(&mut self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
		let mut machine = self.store.get_voting_machine().await?;

		let ballot_paper = BallotPaper::from(vote_form);
		let result = machine.vote(ballot_paper);

		self.store.put_working_machine(machine).await?;

		return Ok(result);
	}
	
	pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		return self.store.get_voting_machine().await;
	}
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeSet;

use crate::{domain::{AttendenceSheet, Scoreboard}, storages::memory::MemoryStore};

use super::*;

	#[tokio::test]
	async fn vote_nul() {
		let candidate_a = Candidate("A".to_string());
		let candidate_b = Candidate("B".to_string());

		let candidates = vec![candidate_a.clone(), candidate_b.clone()];
		let scoreboard = Scoreboard::new(candidates);

		let attendence_sheet = AttendenceSheet(BTreeSet::new());

		let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);
		let store = MemoryStore::new(voting_machine).await.unwrap();

		let mut voting_controller = VotingController::new(store);

		let voter = Voter("Alice".to_string());
		let paper = VoteForm {
			voter: voter.0.clone(),
			candidate: "".to_string()
		};

		let outcome = voting_controller.vote(paper).await.unwrap();

		match outcome {
			VoteOutcome::BlankVote(_voter_result) => {
				let machine = voting_controller.get_voting_machine().await.unwrap();
				let scoreboard = machine.get_scoreboard();

				let candid_a_score = scoreboard.scores.get(&candidate_a)
					.expect("Expected candidate A to have a score");

				assert_eq!(candid_a_score.0, 0);

				let candid_b_score = scoreboard.scores.get(&candidate_b.clone())
					.expect("Expected candidate B to have a score");

				assert_eq!(candid_b_score.0, 0);
				
			}
			_ => panic!("Expected InvalidVote")
		}
	}

	// Fais moi le déjà voté, le vote blanc et le vote valide
	#[tokio::test]
	async fn vote_already_voted() {
		let candidate_a = Candidate("A".to_string());
		let candidate_b = Candidate("B".to_string());

		let candidates = vec![candidate_a.clone(), candidate_b.clone()];
		let scoreboard = Scoreboard::new(candidates);

		let mut attendence_sheet = AttendenceSheet(BTreeSet::new());
		attendence_sheet.0.insert(Voter("Alice".to_string()));

		let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);
		let store = MemoryStore::new(voting_machine).await.unwrap();

		let mut voting_controller = VotingController::new(store);

		let voter = Voter("Alice".to_string());
		let paper = VoteForm {
			voter: voter.0.clone(),
			candidate: "".to_string()
		};

		let outcome = voting_controller.vote(paper).await.unwrap();

		match outcome {
			VoteOutcome::HasAlreadyVoted(_voter_result) => {
				let machine = voting_controller.get_voting_machine().await.unwrap();
				let scoreboard = machine.get_scoreboard();

				let candid_a_score = scoreboard.scores.get(&candidate_a)
					.expect("Expected candidate A to have a score");

				assert_eq!(candid_a_score.0, 0);

				let candid_b_score = scoreboard.scores.get(&candidate_b.clone())
					.expect("Expected candidate B to have a score");

				assert_eq!(candid_b_score.0, 0);
				
			}
			_ => panic!("Expected HasAlreadyVoted")
		}
	}

	#[tokio::test]
	async fn vote_valide() {
		let candidate_a = Candidate("A".to_string());
		let candidate_b = Candidate("B".to_string());

		let candidates = vec![candidate_a.clone(), candidate_b.clone()];
		let scoreboard = Scoreboard::new(candidates);

		let attendence_sheet = AttendenceSheet(BTreeSet::new());

		let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);
		let store = MemoryStore::new(voting_machine).await.unwrap();

		let mut voting_controller = VotingController::new(store);

		let voter = Voter("Alice".to_string());
		let paper = VoteForm {
			voter: voter.0.clone(),
			candidate: candidate_a.0.clone()
		};

		let outcome = voting_controller.vote(paper).await.unwrap();

		match outcome {
			VoteOutcome::AcceptedVote(_voter_result, _candidate_result) => {
				let machine = voting_controller.get_voting_machine().await.unwrap();
				let scoreboard = machine.get_scoreboard();

				let candid_a_score = scoreboard.scores.get(&candidate_a)
					.expect("Expected candidate A to have a score");

				assert_eq!(candid_a_score.0, 1);

				let candid_b_score = scoreboard.scores.get(&candidate_b.clone())
					.expect("Expected candidate B to have a score");

				assert_eq!(candid_b_score.0, 0);
			},
			_ => panic!("Expected AcceptedVote")
		}
	}

	#[tokio::test]
	async fn blank_vote() {
		let candidate_a = Candidate("A".to_string());
		let candidate_b = Candidate("B".to_string());

		let candidates = vec![candidate_a.clone(), candidate_b.clone()];
		let scoreboard = Scoreboard::new(candidates);

		let attendence_sheet = AttendenceSheet(BTreeSet::new());

		let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);
		let store = MemoryStore::new(voting_machine).await.unwrap();

		let mut voting_controller = VotingController::new(store);

		let voter = Voter("Alice".to_string());
		let paper = VoteForm {
			voter: voter.0.clone(),
			candidate: "".to_string()
		};

		let outcome = voting_controller.vote(paper).await.unwrap();

		match outcome {
			VoteOutcome::BlankVote(_voter_result) => {
				let machine = voting_controller.get_voting_machine().await.unwrap();
				let scoreboard = machine.get_scoreboard();

				let candid_a_score = scoreboard.scores.get(&candidate_a)
					.expect("Expected candidate A to have a score");

				assert_eq!(candid_a_score.0, 0);

				let candid_b_score = scoreboard.scores.get(&candidate_b.clone())
					.expect("Expected candidate B to have a score");

				assert_eq!(candid_b_score.0, 0);
			},
			_ => panic!("Expected BlankVote")
		}
	}
}