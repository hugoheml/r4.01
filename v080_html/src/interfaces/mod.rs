use lexicon::Lexicon;

pub mod cli_interface;
pub mod web_interfaces;
pub mod lexicons;
pub mod lexicon;

fn show_vote_outcome(outcome: crate::domain::VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        crate::domain::VoteOutcome::AcceptedVote(voter, candidate) => {
            return format!("{} {} {}", lexicon.vote_of, voter.0, candidate.0);
        },
        crate::domain::VoteOutcome::BlankVote(voter) => {
            return format!("{} {} {}", lexicon.vote_of, voter.0, lexicon.blank_vote);
        },
        crate::domain::VoteOutcome::InvalidVote(voter) => {
            return format!("{} {} {}", lexicon.vote_of, voter.0, lexicon.invalid_vote);
        },
        crate::domain::VoteOutcome::HasAlreadyVoted(voter) => {
            return format!("{} {}", voter.0, lexicon.has_already_voted);
        }
    }
}