use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;
use std::fmt;

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Debug)]
pub struct Voter(pub String);

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Debug)]
pub struct Candidate(pub String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Score(pub usize);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AttendenceSheet(pub Set<Voter>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Scoreboard {
	pub scores: Map<Candidate, Score>,
	pub blank_score: Score,
	pub invalid_score: Score,
}

pub struct BallotPaper {
	pub voter: Voter,
	pub candidate: Option<Candidate>
}

pub enum VoteOutcome {
	AcceptedVote(Voter, Candidate),
	BlankVote(Voter),
	InvalidVote(Voter),
	HasAlreadyVoted(Voter)
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct VotingMachine {
	voters: AttendenceSheet,
	scoreboard: Scoreboard
}

impl Scoreboard {
	pub fn new(candidates: Vec<Candidate>) -> Self {
		let mut scores = Map::new();
		for candidate in candidates {
			scores.insert(candidate, Score(0));
		}

		return Self {
			scores: scores,
			blank_score: Score(0),
			invalid_score: Score(0)
		}
	}
}

impl VotingMachine {
	pub fn new(voters: AttendenceSheet, scoreboard: Scoreboard) -> Self {
		return Self {
			voters: voters,
			scoreboard: scoreboard
		}
	}

	pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
		if self.voters.0.contains(&ballot_paper.voter) {
			return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
		}

		self.voters.0.insert(ballot_paper.voter.clone());

		if ballot_paper.candidate.is_none() {
			self.scoreboard.blank_score.0 += 1;
			return VoteOutcome::BlankVote(ballot_paper.voter);
		}

		let candidate = ballot_paper.candidate.unwrap();
		if !self.scoreboard.scores.contains_key(&candidate) {
			self.scoreboard.invalid_score.0 += 1;
			return VoteOutcome::InvalidVote(ballot_paper.voter);
		}

		self.scoreboard.scores.get_mut(&candidate).map(|score| score.0 += 1);

		return VoteOutcome::AcceptedVote(ballot_paper.voter, candidate);
	}

	pub fn get_scoreboard(&self) -> &Scoreboard {
		return &self.scoreboard;
	}

	pub fn get_voters(&self) -> &AttendenceSheet {
		return &self.voters;
	}

	pub fn recover_from(voters: AttendenceSheet, scoreboard: Scoreboard) -> Self {
		return Self {
			voters: voters,
			scoreboard: scoreboard
		}
	}
}

impl fmt::Display for Candidate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return write!(f, "{}", self.0);
	}
}

impl fmt::Display for Voter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return write!(f, "{}", self.0);
	}
}

impl fmt::Display for Score {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return write!(f, "{}", self.0);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn vote_nul() {
		use super::*;

		let voters = AttendenceSheet(Set::new());
		let candidates = vec![Candidate("A".to_string()), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);
		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let voter = Voter("Alice".to_string());
		let paper = BallotPaper {
			voter: voter.clone(),
			candidate: Some(Candidate("C".to_string()))
		};

		let outcome = voting_machine.vote(paper);

		match outcome {
			VoteOutcome::InvalidVote(voter_result) => {
				assert_eq!(voter_result, voter);
			},
			_ => panic!("Mauvais résultat")
		}
	}

	#[test]
	fn vote_blanc() {
		use super::*;

		let voters = AttendenceSheet(Set::new());
		let candidates = vec![Candidate("A".to_string()), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);
		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let voter = Voter("Alice".to_string());
		let paper = BallotPaper {
			voter: voter.clone(),
			candidate: None
		};

		let outcome = voting_machine.vote(paper);

		match outcome {
			VoteOutcome::BlankVote(voter_result) => {
				assert_eq!(voter_result, voter);
			},
			_ => panic!("Mauvais résultat")
		}
	}

	#[test]
	fn vote_accepte() {
		use super::*;

		let voters = AttendenceSheet(Set::new());

		let candidate_a = Candidate("A".to_string());

		let candidates = vec![candidate_a.clone(), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);
		let mut voting_machine = VotingMachine::new(voters, scoreboard);


		let voter = Voter("Alice".to_string());
		let paper = BallotPaper {
			voter: voter.clone(),
			candidate: Some(candidate_a.clone())
		};

		let outcome = voting_machine.vote(paper);

		match outcome {
			VoteOutcome::AcceptedVote(voter_result, candidate_result) => {
				assert_eq!(voter_result, voter);
				assert_eq!(candidate_result, candidate_a);
			},
			_ => panic!("Mauvais résultat")
		}
	}

	#[test]
	fn already_voted() {
		use super::*;

		let voter = Voter("Alice".to_string());
		let voters = AttendenceSheet(Set::from([voter.clone()]));

		let candidate_a = Candidate("A".to_string());


		let candidates = vec![candidate_a.clone(), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);

		let mut voting_machine = VotingMachine::new(voters, scoreboard);

		let paper = BallotPaper {
			voter: voter.clone(),
			candidate: Some(candidate_a)
		};

		voting_machine.vote(paper);

		let new_paper: BallotPaper = BallotPaper {
			voter: voter.clone(),
			candidate: None
		};

		let outcome = voting_machine.vote(new_paper);

		match outcome {
			VoteOutcome::HasAlreadyVoted(voter_result) => {
				assert_eq!(voter_result, voter);
			},
			_ => panic!("Mauvais résultat")
		}
	}
}