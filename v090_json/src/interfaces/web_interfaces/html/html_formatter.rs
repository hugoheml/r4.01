use maud::{html, Markup};

use crate::{domain::VotingMachine, interfaces::{lexicon::Lexicon, web_interfaces::web_routers::WebRoutes}};

pub fn vote_form(routes: &WebRoutes, lexicon: &Lexicon) -> Markup {
	html! {
        script src="https://unpkg.com/htmx.org@1.9.2" {}
        h2 { (lexicon.ballot_paper) }
        form {            
            label for="voter" { (lexicon.voter) }
            input type="text" id="voter" name="voter" required;
    
            label for="candidate" { (lexicon.candidate) }
            input type="text" id="candidate" name="candidate" required;
    
            button id="vote" hx-post=(routes.vote) hx-target="#outcome" hx-swap="innerHTML" { (lexicon.vote_action) }
        }
        p id="outcome" {}
    }
}

pub fn voting_machine(routes: &WebRoutes, lexicon: &Lexicon, machine: &VotingMachine) -> Markup {
	html! {
        div hx-get=(routes.results) hx-trigger="every 3s" {
            h2 { (lexicon.scores) }
            table {
                @for (candidate, score) in &machine.get_scoreboard().scores {
                    tr {
                        td { (candidate) }
                        td { (score) }
                    }
                }
                tr {
                    td { (lexicon.blank) }
                    td { (machine.get_scoreboard().blank_score ) }
                }
                tr {
                    td { (lexicon.invalid_votes) }
                    td { (machine.get_scoreboard().invalid_score ) }
                }
            }
            h2 { (lexicon.voters) }
            ul {
                li { "Tux" }
            }
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