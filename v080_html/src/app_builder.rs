use std::collections::BTreeSet as Set;

use anyhow::Ok;

use crate::configuration::Configuration;
use crate::domain::AttendenceSheet;
use crate::domain::Candidate;
use crate::domain::Scoreboard;
use crate::domain::Voter;
use crate::domain::VotingMachine;
use crate::interfaces::lexicons::english::ENGLISH_LEXICON;
use crate::interfaces::lexicons::french::FRENCH_LEXICON;
use crate::service::Service;
use crate::services::stdio::StdioService;
use crate::services::tcp::TcpService;
use crate::services::udp::UdpService;
use crate::services::web::WebService;
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

pub async fn handle_lines<Store: Storage, Serv: Service<Store>>(configuration: &Configuration) -> anyhow::Result<()> {
    let voting_machine_init: VotingMachine = create_voting_machine(configuration);
    let store = Store::new(voting_machine_init).await?;
    let voting_controller = VotingController::new(store);

    let language = match configuration.language {
        crate::configuration::LexiconType::Fr => FRENCH_LEXICON,
        crate::configuration::LexiconType::En => ENGLISH_LEXICON
    };

    Serv::new(configuration.port, language, voting_controller).serve().await?;

    Ok(())
}

async fn dispatch_service<Store: Storage + Send + Sync + Clone>(configuration: Configuration) -> anyhow::Result<(), anyhow::Error> {

    match configuration.service {
        crate::configuration::ServiceType::Stdio => {
            handle_lines::<FileStore, StdioService<FileStore>>(&configuration).await
        },
        crate::configuration::ServiceType::Udp => {
            handle_lines::<FileStore, UdpService<FileStore>>(&configuration).await
        },
        crate::configuration::ServiceType::Tcp => {
            handle_lines::<FileStore, TcpService<FileStore>>(&configuration).await
        },
        crate::configuration::ServiceType::Web => {
            handle_lines::<FileStore, WebService>(&configuration).await
        }
    }
}

pub async fn run_app(configuration: Configuration) -> anyhow::Result<()> {

    match configuration.storage {
        crate::configuration::StorageType::File => {
            dispatch_service::<FileStore>(configuration).await
        },
        crate::configuration::StorageType::Memory => {
            dispatch_service::<MemoryStore>(configuration).await
        }
    }
}