use maud::{html, Markup};

use crate::{domain::VotingMachine, interfaces::{lexicon::Lexicon, web_interfaces::web_routers::WebRoutes}};

pub fn vote_form(routes: &WebRoutes, lexicon: &Lexicon) -> Markup {
	html!{
        script src="https://unpkg.com/htmx.org@1.9.2" {}
		h2 { (lexicon.ballot_paper) }
        label for="votant" { (lexicon.voter) }
        input type="text" id="votant" name="votant" required;

        label for="candidat" { (lexicon.candidate) }
        input type="text" id="candidat" name="candidat" required;

        button id="vote" { (lexicon.vote_action) }
	}
}

pub fn voting_machine(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
	html! {
        h2 { (lexicon.scores) }
        table {
            tr {
                td { "Linux" }
                td { "0" }
            }
        }
        h2 { (lexicon.voters) }
        ul {
            li { "Tux" }
        }
	}
}

pub fn index(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
	html! (
		h1 { (lexicon.voting_machine) }
		(vote_form(routes, lexicon))
		(voting_machine(routes, lexicon, machine))
	)
}