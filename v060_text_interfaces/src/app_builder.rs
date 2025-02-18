use std::collections::BTreeSet as Set;

use crate::configuration::Configuration;
use crate::domain::AttendenceSheet;
use crate::domain::Candidate;
use crate::domain::Scoreboard;
use crate::domain::Voter;
use crate::domain::VotingMachine;
use crate::interfaces::cli_interface::handle_line;
use crate::interfaces::lexicons::english::ENGLISH_LEXICON;
use crate::interfaces::lexicons::french::FRENCH_LEXICON;
use crate::storage::Storage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_cases::VotingController;

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let voters: Set<Voter> = Set::new();
    let candidates: Vec<Candidate> = configuration.candidates.iter().map(|candidate| Candidate((*candidate).clone())).collect();

    let attendence_sheet = AttendenceSheet(voters);
    let scoreboard = Scoreboard::new(candidates);

    let voting_machine = VotingMachine::new(attendence_sheet, scoreboard);

    return voting_machine;
}

pub async fn handle_lines<Store: Storage>(configuration: &Configuration) -> anyhow::Result<()> {
    let voting_machine_init: VotingMachine = create_voting_machine(configuration);
    let store = Store::new(voting_machine_init).await?;
    let mut voting_controller = VotingController::new(store);

    
    let language = match configuration.language {
        crate::configuration::LexiconType::Fr => FRENCH_LEXICON,
        crate::configuration::LexiconType::En => ENGLISH_LEXICON
    };

    println!("{}", language.choose_something_to_do);
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        handle_line(&mut voting_controller, &language, &input).await?;
    }
}
pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {

    match configuration.storage {
        crate::configuration::StorageType::File => {
            handle_lines::<FileStore>(&configuration).await
        },
        crate::configuration::StorageType::Memory => {
            handle_lines::<MemoryStore>(&configuration).await
        }
    }
}