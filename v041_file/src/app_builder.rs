use std::collections::BTreeSet as Set;

use crate::configuration::Configuration;
use crate::domain::AttendenceSheet;
use crate::domain::BallotPaper;
use crate::domain::Candidate;
use crate::domain::Scoreboard;
use crate::domain::Voter;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;

fn help() {
    println!("Commandes: ");
    println!("voter <nom> <vote> : Voter pour quelqu'un");
    println!("votants : Afficher la liste des votants");
    println!("scores : fait afficher les scores pour tous les candidats");
}

fn create_voting_machine(configuration: Configuration) -> VotingMachine {
    let voters: Set<Voter> = Set::new();
    let candidates: Vec<Candidate> = configuration.candidates.into_iter().map(|candidate| Candidate(candidate)).collect();

    let attendence_sheet = AttendenceSheet(voters);
    let scoreboard = Scoreboard::new(candidates);

    let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

    return voting_machine;
}

async fn handle_lines<Store: Storage>(configuration: Configuration) -> anyhow::Result<()> {
    let voting_machine_init: VotingMachine = create_voting_machine(configuration);
    let mut store = Store::new(voting_machine_init).await?;

    println!("Machine a vote: Que voulez-vous faire ?");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.clone();
        let mut input_args = input.split_whitespace();

        let command = input_args.next();

        let mut voting_machine: VotingMachine = store.get_voting_machine().await?;

        match command {
            None => {
                println!("Commande invalide");
            }
            Some("voter") => {
                let voter_name_optn: Option<&str> = input_args.next();
                match voter_name_optn {
                    None => {
                        println!("Vous devez utiliser la commande de la façon suivante : voter <nom> <vote>");
                    },
                    Some(voter_name) => {
                        if voter_name == "" {
                            println!("Vous devez utiliser la commande de la façon suivante : voter <nom> <vote>");
                        }

                        let voter = Voter(voter_name.to_string());

                        let candidate_optn: Option<&str> = input_args.next();
                        let mut paper = BallotPaper {
                            voter: voter.clone(),
                            candidate: None
                        };

                        match candidate_optn {
                            None => {},
                            Some(candidate_name) => {
                                if candidate_name == "" {
                                    println!("Vous devez utiliser la commande de la façon suivante : voter <nom> <vote>");
                                    continue;
                                }

                                let candidate = Candidate(candidate_name.to_string());
                                paper = BallotPaper {
                                    voter: voter.clone(),
                                    candidate: Some(candidate)
                                };
                            }
                        }

                        let outcome = voting_machine.vote(paper);

                        match outcome {
                            crate::domain::VoteOutcome::AcceptedVote(voter, candidate) => {
                                println!("Vote de {} pour {}", voter.0, candidate.0);
                            },
                            crate::domain::VoteOutcome::BlankVote(voter) => {
                                println!("Vote de {} blanc", voter.0);
                            },
                            crate::domain::VoteOutcome::InvalidVote(voter) => {
                                println!("Vote de {} nul", voter.0);
                            },
                            crate::domain::VoteOutcome::HasAlreadyVoted(voter) => {
                                println!("{} a déjà voté", voter.0);
                            }
                        }
                    }
                }
            }
            Some("votants") => {
                let voters = voting_machine.get_voters();
                println!("Liste des votants: ");
                for voter in voters.0.iter() {
                    println!("{}", voter.0);
                }
            }   
            Some("scores") => {
                let scores = voting_machine.get_scoreboard();
                println!("Scores: ");
                for (candidate, score) in scores.scores.iter() {
                    println!("{}: {}", candidate.0, score.0);
                }
                println!("Votes blancs: {}", scores.blank_score.0);
                println!("Votes invalides: {}", scores.invalid_score.0);
            }
            Some(_) => {
                help();
            }
        }

        store.put_working_machine(voting_machine).await?;
    }

}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    match configuration.storage {
        crate::configuration::StorageType::File => {
            handle_lines::<FileStore>(configuration).await
        },
        crate::configuration::StorageType::Memory => {
            handle_lines::<MemoryStore>(configuration).await
        }
    }
}