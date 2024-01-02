use parity_scale_codec::Encode as _;
use sp_core::{blake2_256, H256};
use sp_std::str::FromStr;
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
	// dirty hack because I don't know subxt well enough to read all keys
	let member_accounts: Vec<&'static str> = [
		"1363HWTPzDrzAQ6ChFiMU6mP4b6jmQid2ae55JQcKtZnpLGv",
		"16a357f5Sxab3V2ne4emGQvqJaCLeYpTMx3TCjnQhmJQ71DX",
		"121dd6J26VUnBZ8BqLGjANWkEAXSb9mWq1SB7LsS9QNTGFvz",
		"1eTPAR2TuqLyidmPT9rMmuycHVm9s9czu78sePqg2KHMDrE",
		"12hAtDZJGt4of3m2GqZcUCVAjZPALfvPwvtUTFZPQUbdX1Ud",
		"13dbGnwdBYZPMLGkHj3RZX49Xin9DrLXhyNPQkWSe474XJot",
		"124GdqpPvjbqWwsRHjJ4UGYAeCUzGHe4NQgRHeTPSfeCN9tN",
		"1hYiMW8KSfUYChzCQSPGXvMSyKVqmyvMXqohjKr3oU5PCXF",
		"16SDAKg9N6kKAbhgDyxBXdHEwpwHUHs2CNEiLNGeZV55qHna",
		"15ydCCiJb3AfbMQkFHikZxVpg2dUWz58Wtf7ScztD8RRkFAN",
		"15MnsMDx6kavY25y2w2ZUvPgBzzGfV8PRo6Kkmx2vH4wqJE6",
		"16Q4qkRcWd4r8196dVGNLYVfy7H86MJYJBMockPaMigFXCyv",
		"12pRzYaysQz6Tr1e78sRmu9FGB8gu8yTek9x6xwVFFAwXTM8",
		"15akrup6APpRegG1TtWkYVuWHYc37tJ8XPN61vCuHQUi65Mx",
		"156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y",
		"13zbXMsJgQbMkrjai3JUaQmVERZHKLNt42xaGjqJq7mBasDk",
		"14mDeKZ7qp9hqBjjDg51c8BFrf9o69om8piSSRwj2fT5Yb1i",
		"15JTWvBx8mSV1xASTnURwi9WzjrJr8YvUwnzhZzG7B2M8MyW",
		"15DCWHQknBjc5YPFoVj8Pn2KoqrqYywJJ95BYNYJ4Fj3NLqz",
		"15tdgwSWjLj6oGbuC3KhSctVc9FAAJCcrykN9AZQeCixvkqC",
		"1ZYq7qWxvqC1rE4PKnTWwoVJYLtn99p9JM9cUn2WqbWr3Jt",
		"12YzxR5TvGzfMVZNnhAJ5Hwi5zExpRWMKv2MuMwZTrddvgoi",
		"15DCZocYEM2ThYCAj22QE4QENRvUNVrDtoLBVbCm5x4EQncr",
		"153FF3AdrgsKmGGWRUbubvYYqFMHZD9bejxHP89FeCZPYZ3z",
		"1QhVP5qzR2LfXqP77N1JcuwHoY7NH8JVRNFm1hSooE9d4pR",
		"14YDyDZ9o1Nr2hMqLSbjYpr4Wm5s1gux6CvjYZfUTJ4Np3w1",
		"1333zsMafds2sKAr8nG3zwXTCHPYv2Nm6CRgakpu6YVGt7nM",
		"12brenJreRTd6XsBMo3Ptu5VMUUycC53ipDxUy9Mvb4L1s1q",
		"13xS6fK6MHjApLnjdX7TJYw1niZmiXasSN91bNtiXQjgEtNx",
		"136Fm42HZTxUAQD1zDxXuzeF8JgijeDfTSkFBWJKxzVAgQ8K",
		"123SVCkcHnNKyng8EPmaUeay5kKHu1jig99RT21E2cEx5pQF",
		"12MrP337azmkTdfCUKe5XLnSQrbgEKqqfZ4PQC7CZTJKAWR3",
		"12HWjfYxi7xt7EvpTxUis7JoNWF7YCqa19JXmuiwizfwJZY2",
		"1RaxuqWvyd6sdAEiansxmtget47PVcsSR38d9V2uPzKu2vo",
		"15db5ksZgmhWE9U8MDq4wLKUdFivLVBybztWV8nmaJvv3NU1",
		"15qE2YAQCs5Y962RHE7RzNjQxU6Pei21nhkkSM9Sojq1hHps",
		"13fvj4bNfrTo8oW6U8525soRp6vhjAFLum6XBdtqq9yP22E7",
		"12zsKEDVcHpKEWb99iFt3xrTCQQXZMu477nJQsTBBrof5k2h",
		"14ShUZUYUR35RBZW6uVVt1zXDxmSQddkeDdXf1JkMA6P721N",
		"15VsPr7y92ZFAN6zv7ELqC7eeWvJN5GT2kVozRfRuNoEZCsN",
		"15tRXfXoZXkjScB3Awv8s2saPjaicKYAyL1WZ3JP5kycoG9n",
		"1682A5hxfiS1Kn1jrUnMYv14T9EuEnsgnBbujGfYbeEbSK3w",
		"12gMhxHw8QjEwLQvnqsmMVY1z5gFa54vND74aMUbhhwN6mJR",
		"15K1tpRFoFsGvqYU2358GE4hK85zQeiKcYo1HT9pnaepRs4U",
		"1ThiBx5DDxFhoD9GY6tz5Fp4Y7Xn1xfLmDddcoFQghDvvjg",
		"13QdJvnJgfoitjrxESwrCWTaLMN8KvXxufDUucXM6EWGuxqh",
		"126X27SbhrV19mBFawys3ovkyBS87SGfYwtwa8J2FjHrtbmA",
		"14reWPxwQfEyy5nqQEuo9xNAL1CPbCK8TvdANzwTEMH4nxfs",
		"15Sm4Do29Ci2X458Pwv9MJa52aqfQg6t2Qw3QGpEHpCS1SKK",
		"167Y1SbQrwQVNfkNUXtRkocfzVbaAHYjnZPkZRScWPQ46XDb",
		"12rhxeaUeeCkGH5pdkbMGFu2jkgLKKVXEMiCtB6VG1GMbkNu",
		"14DDofWN1JuYK6BTVrCwgqy2AvNr3izFA1BSr9RAdJQPXBbC",
		"14DsLzVyTUTDMm2eP3czwPbH53KgqnQRp3CJJZS9GR7yxGDP",
		"14Ak9rrF6RKHHoLLRUYMnzcvvi1t8E1yAMa7tcmiwUfaqzYK",
		"16aQgRVKfD22NehdzZD2VPoenP2hvx8RS2gUirfk6abiCies",
		"15G1iXDLgFyfnJ51FKq1ts44TduMyUtekvzQi9my4hgYt2hs",
		"13jWdKjMNoMnDUxk4JPVxrQTHFJprxqt36ec3X2L4xQoYJZh",
		"15PCbSw1gdtCG9fp1AdLVEQnovPh7LLYWN9krTDAZYhHirSY",
		"1A13Vuern2hQMFHFxDP9vngPtfjnpnVyCiXgYpaiTYo8qFb",
		"13aYUFHB3umoPoxBEAHSv451iR3RpsNi3t5yBZjX2trCtTp6",
		"14ajTQdrtCA8wZmC4PgD8Y1B2Gy8L4Z3oi2fodxq9FehcFrM",
		"15oLanodWWweiZJSoDTEBtrX7oGfq6e8ct5y5E6fVRDPhUgj",
		"16PNhcMEvGbTYdCht9vLy6gj8qehRehEpb2EfmXB7xShZUVX",
		"13DWAWRTVpkPwWdFDmtUfh73KeCWMiJqEV84xRwpg34EZt8Y",
		"1eK9SC7Z2QFi4es2aKQHAehcZqY7bEiprkpsT483PvVK8KE",
		"15zKd25HoSDPHCn1HjuuHTioXLT7PA2pZQ8gYjFsHW4r96qS",
		"165wJzybiNv9VVUypbNRiK5WPKZABTQ5hCFNr9qTAgNCJR12",
	]
	.to_vec();

	assert_eq!(member_accounts.len(), 67);
	let mut ranked_fellows = Vec::with_capacity(member_accounts.len());

	for member in member_accounts {
		let account = AccountId32::from_str(member)?;
		let fellows_query = collectives::storage().fellowship_collective().members(&account);
		let record = api
			.storage()
			.at_latest()
			.await?
			.fetch(&fellows_query)
			.await?
			.ok_or("Fellowship should have this member")?;
		ranked_fellows.push((account, record.rank));
	}

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

	let ones_batch = RuntimeCall::Utility(UtilityCall::force_batch { calls: ones });
	let ones_encoded = ones_batch.encode();
	let ones_hash = blake2_256(&ones_encoded);
	let ones_len: u32 = ones_encoded.len().try_into().unwrap();
	let ones_preimage = RuntimeCall::Preimage(PreimageCall::note_preimage { bytes: ones_encoded });
	let ones_referendum = RuntimeCall::FellowshipReferenda(FellowshipReferendaCall::submit {
		proposal_origin: Box::new(OriginCaller::FellowshipOrigins(FellowshipOrigins::RetainAt1Dan)),
		proposal: Lookup { hash: H256(ones_hash), len: ones_len },
		enactment_moment: DispatchTime::At(dispatch_block),
	});

	let twos_batch = RuntimeCall::Utility(UtilityCall::force_batch { calls: twos });
	let twos_encoded = twos_batch.encode();
	let twos_hash = blake2_256(&twos_encoded);
	let twos_len: u32 = twos_encoded.len().try_into().unwrap();
	let twos_preimage = RuntimeCall::Preimage(PreimageCall::note_preimage { bytes: twos_encoded });
	let twos_referendum = RuntimeCall::FellowshipReferenda(FellowshipReferendaCall::submit {
		proposal_origin: Box::new(OriginCaller::FellowshipOrigins(FellowshipOrigins::RetainAt2Dan)),
		proposal: Lookup { hash: H256(twos_hash), len: twos_len },
		enactment_moment: DispatchTime::At(dispatch_block),
	});

	let threes_batch = RuntimeCall::Utility(UtilityCall::force_batch { calls: threes });
	let threes_encoded = threes_batch.encode();
	let threes_hash = blake2_256(&threes_encoded);
	let threes_len: u32 = threes_encoded.len().try_into().unwrap();
	let threes_preimage =
		RuntimeCall::Preimage(PreimageCall::note_preimage { bytes: threes_encoded });
	let threes_referendum = RuntimeCall::FellowshipReferenda(FellowshipReferendaCall::submit {
		proposal_origin: Box::new(OriginCaller::FellowshipOrigins(FellowshipOrigins::RetainAt3Dan)),
		proposal: Lookup { hash: H256(threes_hash), len: threes_len },
		enactment_moment: DispatchTime::At(dispatch_block),
	});

	let fours_batch = RuntimeCall::Utility(UtilityCall::force_batch { calls: fours });
	let fours_encoded = fours_batch.encode();
	let fours_hash = blake2_256(&fours_encoded);
	let fours_len: u32 = fours_encoded.len().try_into().unwrap();
	let fours_preimage =
		RuntimeCall::Preimage(PreimageCall::note_preimage { bytes: fours_encoded });
	let fours_referendum = RuntimeCall::FellowshipReferenda(FellowshipReferendaCall::submit {
		proposal_origin: Box::new(OriginCaller::FellowshipOrigins(FellowshipOrigins::RetainAt4Dan)),
		proposal: Lookup { hash: H256(fours_hash), len: fours_len },
		enactment_moment: DispatchTime::At(dispatch_block),
	});

	let host: &'static str = "https://polkadot.js.org/apps/";
	let rpc: &'static str = "wss%3A%2F%2Fpolkadot-collectives-rpc.polkadot.io";
	println!("\nAccept all rank 1:");
	println!(
		"- [Preimage]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(ones_preimage.encode())
	);
	println!(
		"- [Referendum]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(ones_referendum.encode())
	);

	println!("\nAccept all rank 2:");
	println!(
		"- [Preimage]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(twos_preimage.encode())
	);
	println!(
		"- [Referendum]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(twos_referendum.encode())
	);

	println!("\nAccept all rank 3:");
	println!(
		"- [Preimage]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(threes_preimage.encode())
	);
	println!(
		"- [Referendum]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(threes_referendum.encode())
	);

	println!("\nAccept all rank 4:");
	println!(
		"- [Preimage]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(fours_preimage.encode())
	);
	println!(
		"- [Referendum]({}?rpc={}#/extrinsics/decode/0x{})",
		host,
		rpc,
		hex::encode(fours_referendum.encode())
	);

	Ok(())
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
