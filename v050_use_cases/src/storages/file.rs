use std::collections::{BTreeMap, BTreeSet};

use crate::{domain::{AttendenceSheet, Candidate, Score, Scoreboard, Voter, VotingMachine}, storage::Storage};
use async_trait::async_trait;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};

const FILEPATH: &str = "machine.json";

pub struct FileStore {
	filepath: String
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ScoreboardDao {
	scores: BTreeMap<String, usize>,
	blank_score: usize,
	invalid_score: usize
}

impl From<Scoreboard> for ScoreboardDao {
	fn from(scoreboard: Scoreboard) -> Self {
		Self {
			scores: scoreboard.scores.iter().map(|(k, v)| (k.to_string(), v.0)).collect(),
			blank_score: scoreboard.blank_score.0,
			invalid_score: scoreboard.invalid_score.0
		}
	}
}

impl From<ScoreboardDao> for Scoreboard {
	fn from(dao: ScoreboardDao) -> Self {
		Self {
			scores: dao.scores.iter().map(|(k, v)| (Candidate(k.to_string()), Score(*v))).collect(),
			blank_score: Score(dao.blank_score),
			invalid_score: Score(dao.invalid_score)
		}
	}
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct VotingMachineDao {
	voters: BTreeSet<String>,
	scoreboard: ScoreboardDao
}

impl From<VotingMachine> for VotingMachineDao {
	fn from(machine: VotingMachine) -> Self {
		Self {
			voters: machine.get_voters().0.iter().map(|v| v.to_string()).collect(),
			scoreboard: ScoreboardDao::from(machine.get_scoreboard().clone())
		}
	}
}

impl From<VotingMachineDao> for VotingMachine {
	fn from(dao: VotingMachineDao) -> Self {
		Self::recover_from(
			AttendenceSheet(dao.voters.iter().map(|v| Voter(v.clone())).collect()),
			Scoreboard::from(dao.scoreboard)
		)
	}
}

impl FileStore {
	pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self> {

		
		match File::open(filepath).await {
			Ok(_) => {
				
			},
			Err(_) => {
				let machine_str: String = serde_json::to_string(&VotingMachineDao::from(machine))?;

				let mut my_file = File::create(filepath).await?;
				my_file.write_all(&machine_str.as_bytes()).await?;
				my_file.flush().await?;
			}
		}

		return {
			Ok(Self {
				filepath: filepath.to_string()
			})
		}
	}
}

#[async_trait]
impl Storage for FileStore {
	async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
		return FileStore::create(machine, FILEPATH).await;
	}

	async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		let mut my_file = File::open(&self.filepath).await?;
		
		let mut my_slice = vec![];
		my_file.read_to_end(&mut my_slice).await?;

		let dao: VotingMachineDao = serde_json::from_slice(&my_slice)?;
		let machine = VotingMachine::from(dao);

		Ok(machine)
	}

	async fn put_working_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
		let machine = serde_json::to_string(&VotingMachineDao::from(machine))?;

		let mut my_file = File::create(&self.filepath).await?;
		my_file.write_all(&machine.as_bytes()).await?;
		my_file.flush().await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn put_get() {
		let voters = AttendenceSheet(BTreeSet::new());
		let candidates = vec![Candidate("A".to_string()), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);
		let voting_machine = VotingMachine::new(voters, scoreboard);

		let file_store = FileStore::create(voting_machine.clone(), "test.json").await.unwrap();

		let voting_machine2 = file_store.get_voting_machine().await.unwrap();

		assert_eq!(voting_machine, voting_machine2);

		tokio::fs::remove_file("test.json").await.unwrap();
	}

	#[tokio::test]
	async fn file_persistence() {
		let voters = AttendenceSheet(BTreeSet::new());
		let candidates = vec![Candidate("A".to_string()), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);
		let voting_machine = VotingMachine::new(voters, scoreboard);

		let file_store = FileStore::create(voting_machine.clone(), "test2.json").await.unwrap();

		let voting_machine2 = file_store.get_voting_machine().await.unwrap();

		assert_eq!(voting_machine, voting_machine2);

		drop(file_store);

		let file_store = FileStore::create(voting_machine.clone(), "test2.json").await.unwrap();

		let voting_machine2 = file_store.get_voting_machine().await.unwrap();

		assert_eq!(voting_machine, voting_machine2);

		tokio::fs::remove_file("test2.json").await.unwrap();
	}
}