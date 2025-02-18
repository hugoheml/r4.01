/*
use crate::interfaces::lexicon::Lexicon;

pub const FRENCH_LEXICON: Lexicon = Lexicon {
    blank: "blanc",
    candidate: "candidat",
    voter: "électeur",
    choose_something_to_do: "choisissez quelque chose à faire",
    invalid_command: "Commande invalide",
    vote_command_usage: "Vous devez utiliser la commande de la façon suivante : voter <nom> <vote>",
    vote_for_someone: "voter <nom> <vote> : Voter pour quelqu'un",
    show_voters: "votants : Afficher la liste des votants",
    show_scores: "scores : fait afficher les scores pour tous les candidats",
    vote_of: "Vote de",
    blank_vote: "blanc",
    invalid_vote: "nul",
    has_already_voted: "a déjà voté",
    scores: "Scores",
    blank_votes: "Votes blancs",
    invalid_votes: "Votes invalides",
    voters: "Votants",
}; */
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
};