use crate::{mock::*, AgentType, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::ConstU32;

#[test]
fn register_validator_did_works() {
	new_test_ext().execute_with(|| {
		// Register validator DID
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		// Check storage
		let expected_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi".to_vec().try_into().unwrap();
		assert_eq!(
			AiAgents::validator_dids(1),
			Some(expected_did.clone())
		);

		// Check reverse lookup
		assert_eq!(AiAgents::did_to_validator(expected_did), Some(1));

		// Check event
		System::assert_has_event(
			Event::ValidatorDidRegistered {
				validator: 1,
				did: b"did:etrid:director-gizzi".to_vec(),
			}
			.into(),
		);
	});
}

#[test]
fn register_duplicate_validator_did_fails() {
	new_test_ext().execute_with(|| {
		// Register first DID
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		// Try to register same DID from different account
		assert_noop!(
			AiAgents::register_validator_did(
				RuntimeOrigin::signed(2),
				b"did:etrid:director-gizzi".to_vec()
			),
			Error::<Test>::DidAlreadyExists
		);

		// Try to register different DID from same account
		assert_noop!(
			AiAgents::register_validator_did(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-eoj".to_vec()
			),
			Error::<Test>::ValidatorDidAlreadyExists
		);
	});
}

#[test]
fn register_invalid_validator_did_fails() {
	new_test_ext().execute_with(|| {
		// Invalid format (doesn't start with did:etrid:director- or did:etrid:validitynode-)
		assert_noop!(
			AiAgents::register_validator_did(
				RuntimeOrigin::signed(1),
				b"did:etrid:invalid-format".to_vec()
			),
			Error::<Test>::InvalidDidFormat
		);
	});
}

#[test]
fn register_agent_did_works() {
	new_test_ext().execute_with(|| {
		// First register validator DID
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		// Register agent DID (using u8: 0=Compiler)
		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // AgentType::Compiler
			b"http://localhost:4000/agents/compiler".to_vec()
		));

		// Check storage
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		assert!(AiAgents::agents(&agent_did).is_some());

		// Check agent details
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.owner, 1);
		assert_eq!(agent.reputation, 500); // InitialReputation
		assert_eq!(agent.action_count, 0);

		// Check event
		System::assert_has_event(
			Event::AgentDidRegistered {
				validator: 1,
				agent_did: b"did:etrid:director-gizzi:compiler".to_vec(),
				agent_type: 0, // Compiler
			}
			.into(),
		);
	});
}

#[test]
fn register_agent_without_validator_did_fails() {
	new_test_ext().execute_with(|| {
		// Try to register agent without validator DID
		assert_noop!(
			AiAgents::register_agent_did(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				0, // Compiler
				b"http://localhost:4000/agents/compiler".to_vec()
			),
			Error::<Test>::ValidatorDidNotSet
		);
	});
}

#[test]
fn register_agent_with_invalid_did_fails() {
	new_test_ext().execute_with(|| {
		// Register validator DID
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		// Try to register agent with DID not matching validator
		assert_noop!(
			AiAgents::register_agent_did(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-eoj:compiler".to_vec(),
				0, // Compiler
				b"http://localhost:4000/agents/compiler".to_vec()
			),
			Error::<Test>::AgentDidMustBeChild
		);
	});
}

#[test]
fn register_max_agents_works() {
	new_test_ext().execute_with(|| {
		// Register validator DID
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		// Register 6 agents (max) - using u8 agent types
		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:governance".to_vec(),
			1, // Governance
			b"http://localhost:4000".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:runtime".to_vec(),
			2, // Runtime
			b"http://localhost:4000".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:economics".to_vec(),
			3, // Economics
			b"http://localhost:4000".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:security".to_vec(),
			4, // Security
			b"http://localhost:4000".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:oracle".to_vec(),
			5, // Oracle
			b"http://localhost:4000".to_vec()
		));

		// Try to register 7th agent - should fail
		assert_noop!(
			AiAgents::register_agent_did(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:extra".to_vec(),
				0, // Compiler
				b"http://localhost:4000".to_vec()
			),
			Error::<Test>::MaxAgentsReached
		);
	});
}

#[test]
fn report_agent_action_works() {
	new_test_ext().execute_with(|| {
		// Setup: Register validator and agent
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Report successful action
		assert_ok!(AiAgents::report_agent_action(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			b"compile".to_vec(),
			b"{\"success\":true,\"duration\":104.5}".to_vec(),
			true
		));

		// Check agent stats
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.action_count, 1);
		assert_eq!(agent.reputation, 501); // 500 + 1 for success

		// Check event
		System::assert_has_event(
			Event::AgentActionReported {
				agent_did: b"did:etrid:director-gizzi:compiler".to_vec(),
				action: b"compile".to_vec(),
				success: true,
			}
			.into(),
		);
	});
}

#[test]
fn report_failed_action_decreases_reputation() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Report failed action
		assert_ok!(AiAgents::report_agent_action(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			b"compile".to_vec(),
			b"{\"success\":false,\"error\":\"Build failed\"}".to_vec(),
			false
		));

		// Check reputation decreased
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.reputation, 495); // 500 - 5 for failure
	});
}

#[test]
fn low_reputation_causes_slashing() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Report many failed actions to drop reputation below threshold (100)
		for _ in 0..81 {
			// 500 - (81 * 5) = 95
			assert_ok!(AiAgents::report_agent_action(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				b"compile".to_vec(),
				b"failed".to_vec(),
				false
			));
		}

		// Check agent is slashed
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.status, crate::AgentStatus::Slashed);

		// Check slashing event
		System::assert_has_event(
			Event::AgentSlashed {
				agent_did: b"did:etrid:director-gizzi:compiler".to_vec(),
				reason: b"Reputation below threshold".to_vec(),
			}
			.into(),
		);
	});
}

#[test]
fn update_agent_status_works() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Update status to Paused (1)
		assert_ok!(AiAgents::update_agent_status(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			1 // Paused
		));

		// Check status updated
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.status, crate::AgentStatus::Paused);

		// Check event
		System::assert_has_event(
			Event::AgentStatusChanged {
				agent_did: b"did:etrid:director-gizzi:compiler".to_vec(),
				old_status: 0, // Active
				new_status: 1, // Paused
			}
			.into(),
		);
	});
}

#[test]
fn update_agent_status_unauthorized_fails() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Try to update status from different account
		assert_noop!(
			AiAgents::update_agent_status(
				RuntimeOrigin::signed(2),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				1 // Paused
			),
			Error::<Test>::NotAgentOwner
		);
	});
}

#[test]
fn report_action_unauthorized_fails() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Try to report action from different account
		assert_noop!(
			AiAgents::report_agent_action(
				RuntimeOrigin::signed(2),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				b"compile".to_vec(),
				b"result".to_vec(),
				true
			),
			Error::<Test>::NotAgentOwner
		);
	});
}

#[test]
fn cannot_unslash_slashed_agent() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Slash agent by dropping reputation
		for _ in 0..81 {
			assert_ok!(AiAgents::report_agent_action(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				b"compile".to_vec(),
				b"failed".to_vec(),
				false
			));
		}

		// Verify agent is slashed
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.status, crate::AgentStatus::Slashed);

		// Try to unslash by updating status - should fail
		assert_noop!(
			AiAgents::update_agent_status(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				0 // Active
			),
			Error::<Test>::AgentAlreadySlashed
		);
	});
}

#[test]
fn invalid_agent_type_fails() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		// Try to register agent with invalid type (6 is out of range, max is 5)
		assert_noop!(
			AiAgents::register_agent_did(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:invalid".to_vec(),
				6, // Invalid - should be 0-5
				b"http://localhost:4000".to_vec()
			),
			Error::<Test>::InvalidAgentType
		);
	});
}

#[test]
fn reputation_caps_at_1000() {
	new_test_ext().execute_with(|| {
		// Setup
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi".to_vec()
		));

		assert_ok!(AiAgents::register_agent_did(
			RuntimeOrigin::signed(1),
			b"did:etrid:director-gizzi:compiler".to_vec(),
			0, // Compiler
			b"http://localhost:4000".to_vec()
		));

		// Report 600 successful actions (should cap at 1000)
		// Starting at 500, +600 = 1100, but capped at 1000
		for _ in 0..600 {
			assert_ok!(AiAgents::report_agent_action(
				RuntimeOrigin::signed(1),
				b"did:etrid:director-gizzi:compiler".to_vec(),
				b"compile".to_vec(),
				b"success".to_vec(),
				true
			));
		}

		// Check reputation is capped at 1000
		let agent_did: BoundedVec<u8, ConstU32<128>> = b"did:etrid:director-gizzi:compiler".to_vec().try_into().unwrap();
		let agent = AiAgents::agents(&agent_did).unwrap();
		assert_eq!(agent.reputation, 1000);
	});
}

#[test]
fn insufficient_stake_fails() {
	new_test_ext().execute_with(|| {
		// Setup validator with insufficient balance (account 3 has 50, needs 100)
		assert_ok!(AiAgents::register_validator_did(
			RuntimeOrigin::signed(3),
			b"did:etrid:director-poor".to_vec()
		));

		// Try to register agent - should fail due to insufficient stake
		assert_noop!(
			AiAgents::register_agent_did(
				RuntimeOrigin::signed(3),
				b"did:etrid:director-poor:compiler".to_vec(),
				0, // Compiler
				b"http://localhost:4000".to_vec()
			),
			Error::<Test>::InsufficientStake
		);
	});
}
