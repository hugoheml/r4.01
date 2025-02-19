#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
	pub blank: &'static str,
	pub candidate: &'static str,
	pub voter: &'static str,

	pub choose_something_to_do: &'static str,
	pub invalid_command: &'static str,
	pub vote_command_usage: &'static str,
	pub vote_for_someone: &'static str,
	pub show_voters: &'static str,
	pub show_scores: &'static str,
	pub vote_of: &'static str,
	pub blank_vote: &'static str,
	pub invalid_vote: &'static str,
	pub has_already_voted: &'static str,
	pub scores: &'static str,
	pub blank_votes: &'static str,
	pub invalid_votes: &'static str,
	pub voters: &'static str,
	pub vote_action: &'static str,
	pub ballot_paper: &'static str,
	pub voting_machine: &'static str
}