use parity_scale_codec::Encode as _;
use sp_core::{blake2_256, H256};
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_url = "wss://polkadot-collectives-rpc.polkadot.io:443")]
mod collectives {}
use collectives::runtime_types::{
	collectives_polkadot_runtime::{
		fellowship::origins::pallet_origins::Origin as FellowshipOrigins, OriginCaller, RuntimeCall,
	},
	frame_support::traits::{preimages::Bounded::Lookup, schedule::DispatchTime},
	pallet_core_fellowship::pallet::Call as CoreFellowshipCall,
	pallet_preimage::pallet::Call as PreimageCall,
	pallet_referenda::pallet::Call as FellowshipReferendaCall,
	pallet_utility::pallet::Call as UtilityCall,
};

#[tokio::main]
async fn main() {
	if let Err(err) = run().await {
		eprintln!("{err}");
	}
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
	let api =
		OnlineClient::<PolkadotConfig>::from_url("wss://polkadot-collectives-rpc.polkadot.io:443")
			.await?;

	let dispatch_block: u32 = 3_000_000;
	let mut ranked_fellows = Vec::new();

	let fellows = collectives::storage().fellowship_collective().members_iter();
	let mut member_records = api.storage().at_latest().await?.iter(fellows).await?;

	while let Some(Ok((account, record))) = member_records.next().await {
		let key_bytes = &account[..];
		assert_eq!(key_bytes.len(), 72); // 40 byte prefix, 32 byte account id
		let mut account_id: [u8; 32] = [0; 32];
		account_id.copy_from_slice(&key_bytes[40..]);
		let fellow = AccountId32::try_from(account_id)?;
		ranked_fellows.push((fellow, record.rank));
	}

	// currently 67 members
	assert_eq!(ranked_fellows.len(), 67);

	let mut ones: Vec<RuntimeCall> = Vec::new();
	let mut twos: Vec<RuntimeCall> = Vec::new();
	let mut threes: Vec<RuntimeCall> = Vec::new();
	let mut fours: Vec<RuntimeCall> = Vec::new();

	for (fellow, rank) in ranked_fellows {
		match rank {
			1 => ones.push(RuntimeCall::FellowshipCore(CoreFellowshipCall::approve {
				who: fellow,
				at_rank: rank,
			})),
			2 => twos.push(RuntimeCall::FellowshipCore(CoreFellowshipCall::approve {
				who: fellow,
				at_rank: rank,
			})),
			3 => threes.push(RuntimeCall::FellowshipCore(CoreFellowshipCall::approve {
				who: fellow,
				at_rank: rank,
			})),
			4 => fours.push(RuntimeCall::FellowshipCore(CoreFellowshipCall::approve {
				who: fellow,
				at_rank: rank,
			})),
			_ => println!("Member {} skipped due to rank of {}", fellow, rank),
		}
	}

	let (ones_preimage, ones_referendum) =
		generate_calls(ones, FellowshipOrigins::RetainAt1Dan, dispatch_block);
	let (twos_preimage, twos_referendum) =
		generate_calls(twos, FellowshipOrigins::RetainAt2Dan, dispatch_block);
	let (threes_preimage, threes_referendum) =
		generate_calls(threes, FellowshipOrigins::RetainAt3Dan, dispatch_block);
	let (fours_preimage, fours_referendum) =
		generate_calls(fours, FellowshipOrigins::RetainAt4Dan, dispatch_block);

	log(ones_preimage, ones_referendum, 1);
	log(twos_preimage, twos_referendum, 2);
	log(threes_preimage, threes_referendum, 3);
	log(fours_preimage, fours_referendum, 4);

	Ok(())
}

fn generate_calls(
	to_accept: Vec<RuntimeCall>,
	track: FellowshipOrigins,
	dispatch_block: u32,
) -> (RuntimeCall, RuntimeCall) {
	let batch = RuntimeCall::Utility(UtilityCall::force_batch { calls: to_accept });
	let encoded = batch.encode();
	let hash = blake2_256(&encoded);
	let len: u32 = encoded.len().try_into().unwrap();
	let preimage = RuntimeCall::Preimage(PreimageCall::note_preimage { bytes: encoded });
	let referendum = RuntimeCall::FellowshipReferenda(FellowshipReferendaCall::submit {
		proposal_origin: Box::new(OriginCaller::FellowshipOrigins(track)),
		proposal: Lookup { hash: H256(hash), len: len },
		enactment_moment: DispatchTime::At(dispatch_block),
	});
	(preimage, referendum)
}

fn log(preimage: RuntimeCall, referendum: RuntimeCall, rank: u16) {
	let host: &'static str = "https://polkadot.js.org/apps/";
	let rpc: &'static str = "wss%3A%2F%2Fpolkadot-collectives-rpc.polkadot.io";
	println!("\n Accept all rank {}", rank);
	println!(
		"- [Preimage]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(preimage.encode())
	);
	println!(
		"- [Referendum]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(referendum.encode())
	);
}

/*
Approve Weight for use in Transact

weight: {
	refTime: 387,253,000
	proofSize: 69,046
}

Gav
0x3f03f0673d30606ee26672707e4fd2bc8b58d3becb7aba2d5f60add64abb5fea47100600

Basti
0x3f037628a5be63c4d3c8dbb96c2904b1a9682e02831a1af836c7efc808020b92fa630600

Rob
0x3f033c235e80e35082b668682531b9b062fda39a46edb94f884d9122d86885fd5f1b0600

Jaco
0x3f035c5062779d44ea2ab0469e155b8cf3e004fce71b3b3d38263cd9fa9478f12f280500

Pierre
0x3f03522523da8f16bc0b51cfd6e8b113f65f6be19f19681d5d6269cb980f9582c3340500

Arkadiy
0x3f03bc64065524532ed9e805fb0d39a5c0199216b52871168e5e4d0ab612f8797d610500

Sergei
0x3f032e1884c53071526483b14004e894415f02b55fc2e2aef8e1df8ccf7ce5bd55700500

Andre
0x3f039c84f75e0b1b92f6b003bde6212a8b2c9b776f3720f942b33fed8709f103a2680500

Encoded call from Relay Chain to `approve` all:
0x630003000100a50f03242f0000060302943577821a0600903f03f0673d30606ee26672707e4fd2bc8b58d3becb7aba2d5f60add64abb5fea47100600060302943577821a0600903f037628a5be63c4d3c8dbb96c2904b1a9682e02831a1af836c7efc808020b92fa630600060302943577821a0600903f033c235e80e35082b668682531b9b062fda39a46edb94f884d9122d86885fd5f1b0600060302943577821a0600903f035c5062779d44ea2ab0469e155b8cf3e004fce71b3b3d38263cd9fa9478f12f280500060302943577821a0600903f03522523da8f16bc0b51cfd6e8b113f65f6be19f19681d5d6269cb980f9582c3340500060302943577821a0600903f03bc64065524532ed9e805fb0d39a5c0199216b52871168e5e4d0ab612f8797d610500060302943577821a0600903f032e1884c53071526483b14004e894415f02b55fc2e2aef8e1df8ccf7ce5bd55700500060302943577821a0600903f039c84f75e0b1b92f6b003bde6212a8b2c9b776f3720f942b33fed8709f103a2680500

TODO: Check origin and track needed.
*/
