use crate::{domain::{AttendenceSheet, Scoreboard, VotingMachine}, storage::Storage, use_cases::{VoteForm, VotingController}};
use super::{lexicon::Lexicon, show_vote_outcome};

pub async fn handle_line<Store: Storage>(voting_controller: &mut VotingController<Store>, lexicon: &Lexicon, input: &str) -> anyhow::Result<String> {
    let mut input_args = input.split_whitespace();
    let command = input_args.next();

    match command {
        None => {
            return Ok(lexicon.invalid_command.to_string());
        }
        Some("voter") => {
            let voter_name_optn: Option<&str> = input_args.next();
            match voter_name_optn {
                None => {
                    return Ok(lexicon.vote_command_usage.to_string());
                },
                Some(voter_name) => {
                    if voter_name == "" {
                        return Ok(lexicon.vote_command_usage.to_string());
                    }

                    let candidate_optn: Option<&str> = input_args.next();
                    let mut paper = VoteForm {
                        voter: voter_name.to_string(),
                        candidate: "".to_string()
                    };

                    match candidate_optn {
                        None => {},
                        Some(candidate_name) => {
                            paper.candidate = candidate_name.to_string();
                        }
                    }

                    let outcome = voting_controller.vote(paper).await?;
                    let result = show_vote_outcome(outcome, lexicon);

                    return Ok(result.to_string());
                }
            }
        }
        Some("votants") => {
            let voting_machine: VotingMachine = voting_controller.get_voting_machine().await?;
            let voters = voting_machine.get_voters();
            let result = show_attendence_sheet(voters, lexicon);

            return Ok(result.to_string());
        }   
        Some("scores") => {
            let voting_machine: VotingMachine = voting_controller.get_voting_machine().await?;
            let scores = voting_machine.get_scoreboard();
            let result = show_scoreboard(scores, lexicon);

            return Ok(result.to_string());
        }
        Some(_) => {
            return Ok(lexicon.invalid_command.to_string());
        }
    }
}

fn show_scoreboard(scoreboard: &Scoreboard, lexicon: &Lexicon) -> String {
    let mut result = String::new();
    result.push_str(&format!("{}:\n", lexicon.scores));
    for (candidate, score) in scoreboard.scores.iter() {
        result.push_str(&format!("{}: {}\n", candidate.0, score.0));
    }
    result.push_str(&format!("{}: {}\n", lexicon.blank_votes, scoreboard.blank_score.0));
    result.push_str(&format!("{}: {}\n", lexicon.invalid_votes, scoreboard.invalid_score.0));
    
    return result;
}

fn show_attendence_sheet(attendence_sheet: &AttendenceSheet, lexicon: &Lexicon) -> String {
    let mut result = String::new();
    result.push_str(&format!("{}:\n", lexicon.voters));
    for voter in attendence_sheet.0.iter() {
        result.push_str(&format!("{}\n", voter.0));
    }
    
    return result;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{domain::{Candidate, Voter, VotingMachine}, interfaces::lexicons::french::FRENCH_LEXICON, storages::memory::MemoryStore};

    use super::*;

    #[tokio::test]
    async fn test_handle_line_no_command() {
        let line = "";
        let mut voters = BTreeSet::new();
        voters.insert(Voter("Alice".to_string()));
        voters.insert(Voter("Bob".to_string()));
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);
        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert_eq!(result.unwrap().to_string(), FRENCH_LEXICON.invalid_command);
    }

    #[tokio::test]
    async fn test_handle_line_show_voters() {
        let line = "votants";
        let mut voters = BTreeSet::new();
        voters.insert(Voter("Alice".to_string()));
        voters.insert(Voter("Bob".to_string()));
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert!(result.unwrap().contains(FRENCH_LEXICON.voters));
    }

    #[tokio::test]
    async fn test_handle_line_show_scores() {
        let line = "scores";
        let mut voters = BTreeSet::new();
        voters.insert(Voter("Alice".to_string()));
        voters.insert(Voter("Bob".to_string()));
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert!(result.unwrap().contains(FRENCH_LEXICON.scores));
    }

    #[tokio::test]
    async fn test_handle_line_vote() {
        let line = "voter Alice Bob";
        let voters = BTreeSet::new();
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![Candidate("Bob".to_string())]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert_eq!(result.unwrap().to_string(), format!("{} Alice Bob", FRENCH_LEXICON.vote_of));
    }

    #[tokio::test]
    async fn test_handle_line_blank_vote() {
        let line = "voter Alice";
        let mut voters = BTreeSet::new();
        voters.insert(Voter("Bob".to_string()));
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert_eq!(result.unwrap().to_string(), format!("{} Alice {}", FRENCH_LEXICON.vote_of, FRENCH_LEXICON.blank_vote));
    }

    #[tokio::test]
    async fn test_handle_line_missing_voter() {
        let line = "voter";
        let mut voters = BTreeSet::new();
        voters.insert(Voter("Alice".to_string()));
        voters.insert(Voter("Bob".to_string()));
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert_eq!(result.unwrap().to_string(), FRENCH_LEXICON.vote_command_usage);
    }

    #[tokio::test]
    async fn test_handle_line_unknown_command() {
        let line = "unknown";
        let mut voters = BTreeSet::new();
        voters.insert(Voter("Alice".to_string()));
        voters.insert(Voter("Bob".to_string()));
        
        let attendence_sheet = AttendenceSheet(voters);
        let scoreboard: Scoreboard = Scoreboard::new(vec![]);
        let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);
        let store = MemoryStore::new(voting_machine).await.unwrap();
        let mut controller = VotingController::new(store);
        let result = handle_line(&mut controller, &FRENCH_LEXICON, line).await;

        assert_eq!(result.unwrap().to_string(), FRENCH_LEXICON.invalid_command);
    }
}