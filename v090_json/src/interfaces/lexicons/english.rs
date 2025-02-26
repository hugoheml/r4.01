use crate::interfaces::lexicon::Lexicon;

pub const ENGLISH_LEXICON: Lexicon = Lexicon {
	blank: "blank",
	candidate: "candidate",
	voter: "voter",

	choose_something_to_do: "choose something to do",
	invalid_command: "Invalid command",
	vote_command_usage: "You must use the command as follows: vote <name> <vote>",
	vote_for_someone: "vote <name> <vote> : Vote for someone",
	show_voters: "voters : Show the list of voters",
	show_scores: "scores : Show the scores for all candidates",
	vote_of: "Vote of",
	blank_vote: "blank",
	invalid_vote: "invalid",
	has_already_voted: "has already voted",
	scores: "Scores",
	blank_votes: "Blank votes",
	invalid_votes: "Invalid votes",
	voters: "Voters",
	vote_action: "Vote",
	ballot_paper: "Ballot paper",
	voting_machine: "Voting machine"
};