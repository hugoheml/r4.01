use async_trait::async_trait;

use crate::{domain::VotingMachine, storage::Storage};

#[derive(Clone)]
pub struct MemoryStore {
	machine: VotingMachine
}

#[async_trait]
impl Storage for MemoryStore {
	async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
		Ok(Self {
			machine
		})
	}

	async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
		return Ok(self.machine.clone());
	}

	async fn put_working_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
		self.machine = machine;

		return Ok(());
	}
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeSet;

use crate::domain::{AttendenceSheet, Candidate, Scoreboard};

use super::*;

	#[tokio::test]
	async fn test_memory_store() {
		let voters = AttendenceSheet(BTreeSet::new());
		let candidates = vec![Candidate("A".to_string()), Candidate("B".to_string())];
		let scoreboard = Scoreboard::new(candidates);
		let voting_machine = VotingMachine::new(voters, scoreboard);

		let memory_store = MemoryStore::new(voting_machine.clone()).await.unwrap();

		let stored_machine = memory_store.get_voting_machine().await.unwrap();

		assert_eq!(voting_machine, stored_machine);
	}
}