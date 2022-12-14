use sp_finality_grandpa::AuthorityId as GrandpaId;
use hex_literal::hex;
use datahighway_runtime::{
    // opaque::{
    //     Block,
    //     SessionKeys,
    // },
    AuthorityDiscoveryConfig,
    BabeConfig,
    BalancesConfig,
    Block,
    DemocracyConfig,
    TechnicalMembershipConfig,
    ElectionsConfig,
    GenesisConfig,
    GrandpaConfig,
    ImOnlineConfig,
    IndicesConfig,
    SessionConfig,
    SessionKeys,
    StakerStatus,
    StakingConfig,
    SudoConfig,
    SystemConfig,
    TreasuryConfig,
    WASM_BINARY,
};
use module_primitives::{
    constants::currency::{
        DOLLARS,
    },
	types::{
        AccountId,
        Balance,
        Signature,
    },
};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::map::Map;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{
    crypto::{
        UncheckedFrom,
        UncheckedInto,
    },
    sr25519,
    Pair,
    Public,
};
use sp_runtime::traits::{
    IdentifyAccount,
    Verify,
};
pub use sp_runtime::{
    Perbill,
    Permill,
};

// Note this is the URL for the telemetry server
const POLKADOT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm not available".to_string())?;

    let mut properties = Map::new();
    properties.insert("tokenSymbol".into(), "DEV".into());
    properties.insert("tokenDecimals".into(), 18.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
        ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial NPoS authorities
            vec![
                get_authority_keys_from_seed("Alice")
            ],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
            vec![
                // DHX DAO Unlocked Reserves Balance
                // Given a Treasury ModuleId in runtime parameter_types of
                // `py/trsry`, we convert that to its associated address
                // using Module ID" to Address" at https://www.shawntabrizi.com/substrate-js-utilities/,
                // which generates 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z,
                // and find its corresponding hex value by pasting the address into
                // "AccountId to Hex" at that same link to return
                // 6d6f646c70792f74727372790000000000000000000000000000000000000000.
                // But since DataHighway is using an SS58 address prefix of 33 instead of
                // Substrate's default of 42, the address corresponds to
                // 4LTFqiD6H6g8a7ur9WH4RxhWx2givWfK7o5EDed3ai1nYTvk.
                // This is pallet_treasury's account_id.
                //
                // Substrate 2 does not have instantiable support for treasury
                // is only supported in Substrate 3 and was fixed here
                // https://github.com/paritytech/substrate/pull/7058
                //
                // Since we have now updated to Substrate 3, we may transfer funds
                // directly to the Treasury, which will hold the
                // DHX DAO Unlocked Reserves Balance.
                //
                // Note: The original DataHighway Testnet Genesis has used:
                //   5FmxcuFwGK7kPmQCB3zhk3HtxxJUyb3WjxosF8jvnkrVRLUG
                //   hex: a42b7518d62a942344fec55d414f1654bf3fd325dbfa32a3c30534d5976acb21
                //
                // However, the DataHighway Westlake Mainnet will transfer the funds to:
                //   4LTFqiD6H6g8a7ur9WH4RxhWx2givWfK7o5EDed3ai1nYTvk
                //   6d6f646c70792f74727372790000000000000000000000000000000000000000
                //
                // To transfer funds from the Treasury, either the Sudo user needs to
                // call the `forceTransfer` extrinsic to transfer funds from the Treasury,
                // or a proposal is required.
                hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into(),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Dave"),
                // Required otherwise get error when compiling
                // `Stash does not have enough balance to bond`
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            ],
			true,
		),
		// Bootnodes
        vec![],
        // Telemetry Endpoints
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm not available".to_string())?;

    let mut properties = Map::new();
    properties.insert("tokenSymbol".into(), "DEV".into());
    properties.insert("tokenDecimals".into(), 18.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local",
        ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial NPoS authorities
            vec![
                get_authority_keys_from_seed("Alice"),
                get_authority_keys_from_seed("Bob"),
                get_authority_keys_from_seed("Charlie"),
                get_authority_keys_from_seed("Dave"),
                get_authority_keys_from_seed("Eve"),
                get_authority_keys_from_seed("Ferdie"),
            ],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
            vec![
                // Endow this account with the DHX DAO Unlocked Reserves Balance
                // 4LTFqiD6H6g8a7ur9WH4RxhWx2givWfK7o5EDed3ai1nYTvk
                hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into(),
                // Endow these accounts with a balance so they may bond as authorities
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Dave"),
                get_account_id_from_seed::<sr25519::Public>("Eve"),
                get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                // Required otherwise get error when compiling
                // `Stash does not have enough balance to bond`
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
            ],
			true,
		),
        // Bootnodes
        vec![
            // Note: The local node identity that is shown when you start the bootnode
            // with the following flags and options is
            // `12D3KooWKS7jU8ti7S5PDqCNWEj692eUSK3DLssHNwTQsto9ynVo`:
            // ./target/release/datahighway \
            //   ...
            //   --alice \
            //   --node-key 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee \
            //   ...
            // Since it is an IP address we use `/ip4/`, whereas if it were a domain we'd use `/dns/`
            "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWKS7jU8ti7S5PDqCNWEj692eUSK3DLssHNwTQsto9ynVo"
                .parse()
                .unwrap(),
        ],
        // Telemetry Endpoints
        Some(
            TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Polkadot telemetry url is valid; qed"),
        ),
        // Protocol ID
        Some("dhx-test"),
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
	))
}

// WARNING: The purpose of this testnet is for initial experiementation with
// multiple validator nodes where chaos such as bricking the chain is permitted,
// to avoid potentially bricking the DataHighway Harbour Testnet and impacting users.
pub fn datahighway_testnet_brickable_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm binary not available".to_string())?;

    let mut properties = Map::new();
    properties.insert("tokenSymbol".into(), "BRI".into());
    properties.insert("tokenDecimals".into(), 18.into());

    Ok(ChainSpec::from_genesis(
        // Name
        "DataHighway Brickable Testnet",
        // ID
        "brickable",
        ChainType::Live,
        move || testnet_genesis(
            wasm_binary,
            // Initial NPoS authorities
            // Note: stash must be before controller
            vec![
                // authority #1
                (
                    // stash
                    hex!["20ee614cc59285dcbca2b4d50c2e20490a87370d4de15baeda649e3538005d4f"].into(),
                    // cont
                    hex!["ba75230fdee3ff9f069bcf8047a52f0655ea5053a04e7de509e3b1d019c2b511"].into(),
                    // gran
                    hex!["edb0bfee980d12609f9641e1720ee4b2d4bee53e052c71e13580ee8f144a361c"]
                        .unchecked_into(),
                    // babe
                    hex!["38ac520a6d9e78538d9351526d98eba4e4cdbabac0329f7be20146f69775964b"]
                        .unchecked_into(),
                    // im_online
                    hex!["02ac967466f3a26e6160bc89d9f41bf0c919a36329e53c69427038f222eae917"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["24a4f1c3c73f19467462f575cc2dda90076bdf0ce7e012f76ec255edb3e2ba54"]
                        .unchecked_into(),
                ),
                // authority #2
                (
                    // stash
                    hex!["18c8fc8aac47703f11e022b304a78fcff4b06f4723e6a5e748e7ae15106a8c06"].into(),
                    // cont
                    hex!["cc135d9509883c963bc58bf987c5f66867a3bf4a09c65b30bfb7654e88178c4d"].into(),
                    // gran
                    hex!["4bbcc0bb7e3f10a8ad097f00c8cd87ab647904e87c9103a46a3db20e74507bea"]
                        .unchecked_into(),
                    // babe
                    hex!["2eac5b0989874724c036a0df4db41adeff9de77e23fb2a136056535722cce84d"]
                        .unchecked_into(),
                    // im_online
                    hex!["b4827253f5bb96ff14cebd31ebe5d4cd1f7300448eddc115f5d9f8b1fde3e404"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["801ffc2fa88b7a2cc58d0e3dfb975d27b56c8de3bb286bc33a86b549679ba23f"]
                        .unchecked_into(),
                ),
                // authority #3
                (
                    // stash
                    hex!["26f3fc47ec49a2981a95352dba050435f135d6a76f25cc489004e2ae098c8c5e"].into(),
                    // cont
                    hex!["6eccb2e1ee65d161e85b45b45a2f5d859dd214c075d5c2a0aa35174281af3e76"].into(),
                    // gran
                    hex!["f8aaa52bc3a0b168fb1bdcfd3d4fe4220cfd893593371c4c2d21defd605ffa4e"]
                        .unchecked_into(),
                    // babe
                    hex!["78013a5320fa1b96e4505472f98948e7305af5385fe2e56ef3f08559975d544c"]
                        .unchecked_into(),
                    // im_online
                    hex!["52540f73318658212a94ee627aa1e90cb7e0331a4aac5f7e844d4e704939b330"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["947f3db6b336ec959c49cb3be516dbaaa4f281d4d63ec794e5e9be63ca0ee165"]
                        .unchecked_into(),
                ),
                // authority #4
                (
                    // stash
                    hex!["3e41a44cee0ade04ae35e7c96bfb2e4070c605aa6d6dcab77130afa594eced68"].into(),
                    // cont
                    hex!["c450c238e0bba0afe639c4c9e1b0f254b9acb193c9e7c390938e229446ac6161"].into(),
                    // gran
                    hex!["5526a6397e13b10a1d2b27114ad8322b8339cdbff9e47867cc11f500783d26ef"]
                        .unchecked_into(),
                    // babe
                    hex!["fab99fbe39614ec041e79e18ba95d5826333a5deece3fd8ad59fcd59c262554d"]
                        .unchecked_into(),
                    // im_online
                    hex!["a0e8c1d5e5dfd4d4626cc425e6fbd3b55a36cf6632e88ea40739e40c27ed3c36"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["2c93879839c514b0e845939e4209810e40bd9ff2ceecaf44a382c08f0359c778"]
                        .unchecked_into(),
                ),
                // authority #5
                (
                    // stash
                    hex!["ce3b5b27aca4be7f957a91a1704ae2ceb8910241f88058c04ef2b6df9a8adb18"].into(),
                    // cont
                    hex!["0213b3d5ea6e0d760fa7183defcef9ae139d8c5728faf6eba22c2cf5c119b838"].into(),
                    // gran
                    hex!["fdb309f030d63d422cdbaf079ade8fbc580aa7debf6c8a18b7db4c097ae1e854"]
                        .unchecked_into(),
                    // babe
                    hex!["a02d7d4b0cf52fa8f00601cc4c82805c48aad2836a0b9c87078a17a4a99aef6a"]
                        .unchecked_into(),
                    // im_online
                    hex!["f4f3b8f5fac4208cf231d3f2ac50e4efd91319eb5e8a65ce393564b542d80132"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["ce3fe564530cbd654871f63e43cc310f9fce59b74d897769dd4edb072420a530"]
                        .unchecked_into(),
                ),
            ],
            // Sudo account
            // 4MF7atBumtP8vGUGG1888e798TCYfXrHMNt52BxW8P3CQNpm
            hex!["9068e3ce9b1055605a3bc4120e697b576c2ac6a13ee6f6ab751ad82e79eb4957"].into(),
            // Pre-funded accounts
            vec![
                // Endow the Sudo account to cover transaction fees
                hex!["9068e3ce9b1055605a3bc4120e697b576c2ac6a13ee6f6ab751ad82e79eb4957"].into(),
                // Endow the Treasury account with the DHX DAO Unlocked Reserves Balance
                // 4LTFqiD6H6g8a7ur9WH4RxhWx2givWfK7o5EDed3ai1nYTvk
                hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into(),
                // Endow these accounts with a balance so they may bond as authorities.
                // IMPORTANT: All authorities must be included in the list below so they have
                // an account balance to avoid session error
                // `assertion failed: frame_system::Module::<T>::inc_consumers(&account).is_ok()`

                // authority #1
                hex!["20ee614cc59285dcbca2b4d50c2e20490a87370d4de15baeda649e3538005d4f"].into(),
                hex!["ba75230fdee3ff9f069bcf8047a52f0655ea5053a04e7de509e3b1d019c2b511"].into(),
                hex!["edb0bfee980d12609f9641e1720ee4b2d4bee53e052c71e13580ee8f144a361c"].into(),
                hex!["38ac520a6d9e78538d9351526d98eba4e4cdbabac0329f7be20146f69775964b"].into(),
                hex!["02ac967466f3a26e6160bc89d9f41bf0c919a36329e53c69427038f222eae917"].into(),
                hex!["24a4f1c3c73f19467462f575cc2dda90076bdf0ce7e012f76ec255edb3e2ba54"].into(),

                // authority #2
                hex!["18c8fc8aac47703f11e022b304a78fcff4b06f4723e6a5e748e7ae15106a8c06"].into(),
                hex!["cc135d9509883c963bc58bf987c5f66867a3bf4a09c65b30bfb7654e88178c4d"].into(),
                hex!["4bbcc0bb7e3f10a8ad097f00c8cd87ab647904e87c9103a46a3db20e74507bea"].into(),
                hex!["2eac5b0989874724c036a0df4db41adeff9de77e23fb2a136056535722cce84d"].into(),
                hex!["b4827253f5bb96ff14cebd31ebe5d4cd1f7300448eddc115f5d9f8b1fde3e404"].into(),
                hex!["801ffc2fa88b7a2cc58d0e3dfb975d27b56c8de3bb286bc33a86b549679ba23f"].into(),

                // authority #3
                hex!["26f3fc47ec49a2981a95352dba050435f135d6a76f25cc489004e2ae098c8c5e"].into(),
                hex!["6eccb2e1ee65d161e85b45b45a2f5d859dd214c075d5c2a0aa35174281af3e76"].into(),
                hex!["f8aaa52bc3a0b168fb1bdcfd3d4fe4220cfd893593371c4c2d21defd605ffa4e"].into(),
                hex!["78013a5320fa1b96e4505472f98948e7305af5385fe2e56ef3f08559975d544c"].into(),
                hex!["52540f73318658212a94ee627aa1e90cb7e0331a4aac5f7e844d4e704939b330"].into(),
                hex!["947f3db6b336ec959c49cb3be516dbaaa4f281d4d63ec794e5e9be63ca0ee165"].into(),

                // authority #4
                hex!["3e41a44cee0ade04ae35e7c96bfb2e4070c605aa6d6dcab77130afa594eced68"].into(),
                hex!["c450c238e0bba0afe639c4c9e1b0f254b9acb193c9e7c390938e229446ac6161"].into(),
                hex!["5526a6397e13b10a1d2b27114ad8322b8339cdbff9e47867cc11f500783d26ef"].into(),
                hex!["fab99fbe39614ec041e79e18ba95d5826333a5deece3fd8ad59fcd59c262554d"].into(),
                hex!["a0e8c1d5e5dfd4d4626cc425e6fbd3b55a36cf6632e88ea40739e40c27ed3c36"].into(),
                hex!["2c93879839c514b0e845939e4209810e40bd9ff2ceecaf44a382c08f0359c778"].into(),

                // authority #5
                hex!["ce3b5b27aca4be7f957a91a1704ae2ceb8910241f88058c04ef2b6df9a8adb18"].into(),
                hex!["0213b3d5ea6e0d760fa7183defcef9ae139d8c5728faf6eba22c2cf5c119b838"].into(),
                hex!["fdb309f030d63d422cdbaf079ade8fbc580aa7debf6c8a18b7db4c097ae1e854"].into(),
                hex!["a02d7d4b0cf52fa8f00601cc4c82805c48aad2836a0b9c87078a17a4a99aef6a"].into(),
                hex!["f4f3b8f5fac4208cf231d3f2ac50e4efd91319eb5e8a65ce393564b542d80132"].into(),
                hex!["ce3fe564530cbd654871f63e43cc310f9fce59b74d897769dd4edb072420a530"].into(),
            ],
            true,
        ),
        vec![
            "/ip4/3.67.117.245/tcp/30333/p2p/12D3KooWBjSUFeT4RrkaTNH5da5LEzZ7M8KNQc9q5s1biNvBD42c"
                .parse()
                .unwrap(),
        ],
        // Telemetry Endpoints
        Some(
            TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Polkadot telemetry url is valid; qed"),
        ),
        // Protocol ID
        Some("dhx-test-brickable"),
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

pub fn datahighway_testnet_harbour_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm binary not available".to_string())?;

    let mut properties = Map::new();
    properties.insert("tokenSymbol".into(), "HBR".into());
    properties.insert("tokenDecimals".into(), 18.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"DataHighway Harbour Testnet",
		// ID
		"harbour",
        ChainType::Live,
		// TODO: regenerate alphanet according to babe-grandpa consensus
		// subkey inspect "$SECRET"
		// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done;
		// done for i in 1 2 3 4; do for j in babe; do subkey inspect
		// --scheme=sr25519 "$SECRET//$i//$j"; done; done for i in 1 2 3 4; do
		// for j in grandpa; do subkey inspect --scheme=ed25519 "$SECRET//$i//$j"; done; done
		move || testnet_genesis(
			wasm_binary,
			// Initial NPoS authorities
            vec![
                // authority #1
                (
                    // stash
                    hex!["f64bae0f8fbe2eb59ff1c0ff760a085f55d69af5909aed280ebda09dc364d443"].into(),
                    // cont
                    hex!["ca907b74f921b74638eb40c289e9bf1142b0afcdb25e1a50383ab8f9d515da0d"].into(),
                    // gran
                    hex!["6a9da05f3e07d68bc29fb6cf9377a1537d59f082f49cb27a47881aef9fbaeaee"]
                        .unchecked_into(),
                    // babe
                    hex!["f2bf53bfe43164d88fcb2e83891137e7cf597857810a870b4c24fb481291b43a"]
                        .unchecked_into(),
                    // im_online
                    hex!["ee834a837b99eb433e48bd64e5604e3a5a7f782e9ca11311832cd8b89a2dd010"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["ae84ff3133f827bd8e5185f34aae89658f86f1ae13e401de2fa3703e7a6a3c6d"]
                        .unchecked_into(),
                ),
                // authority #2
                (
                    // stash
                    hex!["420a7b4a8c9f2388eded13c17841d2a0e08ea7c87eda84310da54f3ccecd3931"].into(),
                    // cont
                    hex!["ae69db7838fb139cbf4f93bf877faf5bbef242f3f5aac6eb4f111398e9385e7d"].into(),
                    // gran
                    hex!["9af1908ac74b042f4be713e10dcf6a2def3770cfce58951c839768e7d6bbcd8e"]
                        .unchecked_into(),
                    // babe
                    hex!["1e91a7902c89289f97756c4e20c0e9536f34de61c7c21af7773d670b0e644030"]
                        .unchecked_into(),
                    // im_online
                    hex!["267cff7dc3de158a584e2c20647006b1e753e1df13b65d4e725c8e23575ff74c"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["8243b8b785a10949839e1958c5d77f9d954912f734cbfb3e0048810e8e0eb20b"]
                        .unchecked_into(),
                ),
                // authority #3
                (
                    // stash
                    hex!["ceecb6cc08c20ff44052ff19952a810d08363aa26ea4fb0a64a62a4630d37f28"].into(),
                    // cont
                    hex!["7652b25328d78d264aef01184202c9771b55f5b391359309a2559ef77fbbb33d"].into(),
                    // gran
                    hex!["b8902681768fbda7a29666e1de8a18f5be3c778d92cf29139959a86e6bff13e7"]
                        .unchecked_into(),
                    // babe
                    hex!["aaabcb653ce5dfd63035430dba10ce9aed5d064883b9e2b19ec5d9b26a457f57"]
                        .unchecked_into(),
                    // im_online
                    hex!["2003bccdbc7d1016ee647856b6cf178fbb9a0f36d841dc9483a39f63cd912e0e"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["bc2e58bf62bc0f49a6a324cd93920f01619a8917d3d9c960f4201cb6cf11721b"]
                        .unchecked_into(),
                ),
                // authority #4
                (
                    // stash
                    hex!["68bac5586028dd40db59a7becec349b42cd4229f9d3c31875c3eb7a57241cd42"].into(),
                    // cont
                    hex!["eec96d02877a45fa524fcee1c6b7c849cbdc8cee01a95f5db168c427ae766849"].into(),
                    // gran
                    hex!["f4807d86cca169a81d42fcf9c7abddeff107b0a73e9e7a809257ac7e4a164741"]
                        .unchecked_into(),
                    // babe
                    hex!["a49ac1053a40a2c7c33ffa41cb285cef7c3bc9db7e03a16d174cc8b5b5ac0247"]
                        .unchecked_into(),
                    // im_online
                    hex!["a6e8669d0916754c654f1adb57d7b28a8f1a7a58794f9bdd4334135458cbab5c"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["06140939946d86bd67d4fc3ab8eee4a9bddd669cd464ace4f5256aa4d9469e19"]
                        .unchecked_into(),
                ),
                // authority #5
                (
                    // stash
                    hex!["181319cac9b915a33586196cb1ed64b7f37f9d50ae9f5c04b0d92ecdb342cd0a"].into(),
                    // cont
                    hex!["80c17c61a1f2f6252b23677510cbd0c3f3ad1ad8694dba9170e98ab430100a7f"].into(),
                    // gran
                    hex!["a240fbd4575d03b4c62e2d2d546327e393db5fc508bc92203fba354a3232f006"]
                        .unchecked_into(),
                    // babe
                    hex!["2ed0676546bc839b77af6e2b084b549403081b1840d4d777a8623f2c44bd0d3e"]
                        .unchecked_into(),
                    // im_online
                    hex!["5209213d7d09853295062d63199cb272888a678a4a2bc52aefd62248b6ff9c02"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["ae641bd93c58d5235054b3a498794b1d18213b9818b7c12eaec596dcb283235f"]
                        .unchecked_into(),
                ),
                // authority #6
                (
                    // stash
                    hex!["04034be760fa29265c5df75891253c28add3d9dc6acc592ab287e4b4e2bdcb13"].into(),
                    // cont
                    hex!["d0dca20158075b94a46aede114a3180d762a7deb9723baa5b3881bda19335716"].into(),
                    // gran
                    hex!["16dcf1c6f2c9c37312d34f1f418e758eb9c97cbaf9ef10b06b6f3b4b4b33f724"]
                        .unchecked_into(),
                    // babe
                    hex!["862f4e806774375fe3695fcd715474bff065f91d2f633096a06acf1167fe8d60"]
                        .unchecked_into(),
                    // im_online
                    hex!["209f13424140c5661d4fd6e13f856c1394a167d1631a47d508afd6dd35463c2e"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["d4ab6cb499d4cf91ea01390b8ceee56e4c50f59a8243a66cdabd9892772edb67"]
                        .unchecked_into(),
                ),
                // authority #7
                (
                    // stash
                    hex!["1e7171d63c29cf75a022f9c7cf817774eb9976a481991fcd7a28c7286d5ec34e"].into(),
                    // cont
                    hex!["4eba5183abe641a421249343037087f48cf3eba9c2628d566bd0e347f619db4a"].into(),
                    // gran
                    hex!["3a592141f7aaef64444aa506a590fdf5769834b74b47f5087eda97ac6833b23f"]
                        .unchecked_into(),
                    // babe
                    hex!["b4801d80cd15b61d9774bf163c60d2068606f2f00f2ff4ee1f7a720062f2f775"]
                        .unchecked_into(),
                    // im_online
                    hex!["b23f016ff3adc74468775edc594c887cfdb0636468f6a522ec41a434a5367969"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["3025a89a305f6e1a5599b3b9f551f907950c97d8e7614b6c248a26e36e4e7d2a"]
                        .unchecked_into(),
                ),
                // authority #8
                (
                    // stash
                    hex!["3af49d16869a494380772e47fb2d92f114c3190e291ed655c3bf9b9b5df52e42"].into(),
                    // cont
                    hex!["98758865268f1a6f7c1307b6ffe18cb764ce9d57561c97423ac145c8d84a4957"].into(),
                    // gran
                    hex!["14d83dc11288148747df1f92d11ad4c5b42dbb12f4aa69f679b36f1f84d41ae0"]
                        .unchecked_into(),
                    // babe
                    hex!["6e239d6162484e5c80efcb4762a98e211e11096f5b71bea251ff5732e3c39244"]
                        .unchecked_into(),
                    // im_online
                    hex!["aa7b10d9bc12874940f92d98f99e7c817c2f86e0630847e3fa124035c6e69a69"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["1006efa023a6729f0cb8908387e847c42f064d9fc8fe71f1bd869a18fc675f56"]
                        .unchecked_into(),
                ),
                // authority #9
                (
                    // stash
                    hex!["603af684fbf8a984af4cc7e147673b576f91da1b3f55e0835a5655a4470e7f22"].into(),
                    // cont
                    hex!["b00da5291fc18c973700c7aeab0ca13bdb9f3e48127657c7c9273e75eb7eb27d"].into(),
                    // gran
                    hex!["c95b718120a73e30ba70bb2a9d369eeb87ed1f5708f21e66b6cb5d7bcfb8c8f7"]
                        .unchecked_into(),
                    // babe
                    hex!["48be9f059c8363935514c7cfe3a4f096a77f101a112539057789c9c4fc54a14d"]
                        .unchecked_into(),
                    // im_online
                    hex!["805a13b47ac6089a144618a54a0a1f1dd5da9c8bcdf6f54390d882b8d8ff616c"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["18e905ae2fbe168ebc14b6682179255ea6410733a0d52e69ed49f3717d73236c"]
                        .unchecked_into(),
                ),
                // authority #10
                (
                    // stash
                    hex!["322b80c5529a20a1da5702acfb78879211eed2110d687ca001309c1c6d57030f"].into(),
                    // cont
                    hex!["48357721e05e42c153e3f33e739b56c4f711762cb45c5463c0b0891bd49fa64b"].into(),
                    // gran
                    hex!["f44fcaa91171530462d0d43225354d09c0e64fc9ac7e6bed017279947d6a4785"]
                        .unchecked_into(),
                    // babe
                    hex!["0233e16df3c3fbed4dcc5cf47e694ed484373899d712665c39ef4441d2fae040"]
                        .unchecked_into(),
                    // im_online
                    hex!["2ec48f57ff1730098daf6a65addefce46f22bff0a5b58b098f306f13b01c145d"]
                        .unchecked_into(),
                    // authority_discovery
                    hex!["886e84efd495efa7b46a8b9a9c45a4e02174ba5b94bdd32381a1bebbdfaf7c77"]
                        .unchecked_into(),
                ),
            ],
			// Sudo account
            hex!["3c917f65753cd375582a6d7a1612c8f01df8805f5c8940a66e9bda3040f88f5d"].into(),
			// Pre-funded accounts
            vec![
                // Endow the Sudo account to cover transaction fees
                hex!["3c917f65753cd375582a6d7a1612c8f01df8805f5c8940a66e9bda3040f88f5d"].into(),
                // Endow the Treasury account with the DHX DAO Unlocked Reserves Balance
                // 4LTFqiD6H6g8a7ur9WH4RxhWx2givWfK7o5EDed3ai1nYTvk
                hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into(),
                // Endow these accounts with a balance so they may bond as authorities.
                // IMPORTANT: All authorities must be included in the list below so they have
                // an account balance to avoid session error
                // `assertion failed: frame_system::Module::<T>::inc_consumers(&account).is_ok()`

                // authority #1
                hex!["f64bae0f8fbe2eb59ff1c0ff760a085f55d69af5909aed280ebda09dc364d443"].into(),
                hex!["ca907b74f921b74638eb40c289e9bf1142b0afcdb25e1a50383ab8f9d515da0d"].into(),
                hex!["6a9da05f3e07d68bc29fb6cf9377a1537d59f082f49cb27a47881aef9fbaeaee"].into(),
                hex!["f2bf53bfe43164d88fcb2e83891137e7cf597857810a870b4c24fb481291b43a"].into(),
                hex!["ee834a837b99eb433e48bd64e5604e3a5a7f782e9ca11311832cd8b89a2dd010"].into(),
                hex!["ae84ff3133f827bd8e5185f34aae89658f86f1ae13e401de2fa3703e7a6a3c6d"].into(),

                // authority #2
                hex!["420a7b4a8c9f2388eded13c17841d2a0e08ea7c87eda84310da54f3ccecd3931"].into(),
                hex!["ae69db7838fb139cbf4f93bf877faf5bbef242f3f5aac6eb4f111398e9385e7d"].into(),
                hex!["9af1908ac74b042f4be713e10dcf6a2def3770cfce58951c839768e7d6bbcd8e"].into(),
                hex!["1e91a7902c89289f97756c4e20c0e9536f34de61c7c21af7773d670b0e644030"].into(),
                hex!["267cff7dc3de158a584e2c20647006b1e753e1df13b65d4e725c8e23575ff74c"].into(),
                hex!["8243b8b785a10949839e1958c5d77f9d954912f734cbfb3e0048810e8e0eb20b"].into(),

                // authority #3
                hex!["ceecb6cc08c20ff44052ff19952a810d08363aa26ea4fb0a64a62a4630d37f28"].into(),
                hex!["7652b25328d78d264aef01184202c9771b55f5b391359309a2559ef77fbbb33d"].into(),
                hex!["b8902681768fbda7a29666e1de8a18f5be3c778d92cf29139959a86e6bff13e7"].into(),
                hex!["aaabcb653ce5dfd63035430dba10ce9aed5d064883b9e2b19ec5d9b26a457f57"].into(),
                hex!["2003bccdbc7d1016ee647856b6cf178fbb9a0f36d841dc9483a39f63cd912e0e"].into(),
                hex!["bc2e58bf62bc0f49a6a324cd93920f01619a8917d3d9c960f4201cb6cf11721b"].into(),

                // authority #4
                hex!["68bac5586028dd40db59a7becec349b42cd4229f9d3c31875c3eb7a57241cd42"].into(),
                hex!["eec96d02877a45fa524fcee1c6b7c849cbdc8cee01a95f5db168c427ae766849"].into(),
                hex!["f4807d86cca169a81d42fcf9c7abddeff107b0a73e9e7a809257ac7e4a164741"].into(),
                hex!["a49ac1053a40a2c7c33ffa41cb285cef7c3bc9db7e03a16d174cc8b5b5ac0247"].into(),
                hex!["a6e8669d0916754c654f1adb57d7b28a8f1a7a58794f9bdd4334135458cbab5c"].into(),
                hex!["06140939946d86bd67d4fc3ab8eee4a9bddd669cd464ace4f5256aa4d9469e19"].into(),

                // authority #5
                hex!["181319cac9b915a33586196cb1ed64b7f37f9d50ae9f5c04b0d92ecdb342cd0a"].into(),
                hex!["80c17c61a1f2f6252b23677510cbd0c3f3ad1ad8694dba9170e98ab430100a7f"].into(),
                hex!["a240fbd4575d03b4c62e2d2d546327e393db5fc508bc92203fba354a3232f006"].into(),
                hex!["2ed0676546bc839b77af6e2b084b549403081b1840d4d777a8623f2c44bd0d3e"].into(),
                hex!["5209213d7d09853295062d63199cb272888a678a4a2bc52aefd62248b6ff9c02"].into(),
                hex!["ae641bd93c58d5235054b3a498794b1d18213b9818b7c12eaec596dcb283235f"].into(),

                // authority #6
                hex!["04034be760fa29265c5df75891253c28add3d9dc6acc592ab287e4b4e2bdcb13"].into(),
                hex!["d0dca20158075b94a46aede114a3180d762a7deb9723baa5b3881bda19335716"].into(),
                hex!["16dcf1c6f2c9c37312d34f1f418e758eb9c97cbaf9ef10b06b6f3b4b4b33f724"].into(),
                hex!["862f4e806774375fe3695fcd715474bff065f91d2f633096a06acf1167fe8d60"].into(),
                hex!["209f13424140c5661d4fd6e13f856c1394a167d1631a47d508afd6dd35463c2e"].into(),
                hex!["d4ab6cb499d4cf91ea01390b8ceee56e4c50f59a8243a66cdabd9892772edb67"].into(),

                // authority #7
                hex!["1e7171d63c29cf75a022f9c7cf817774eb9976a481991fcd7a28c7286d5ec34e"].into(),
                hex!["4eba5183abe641a421249343037087f48cf3eba9c2628d566bd0e347f619db4a"].into(),
                hex!["3a592141f7aaef64444aa506a590fdf5769834b74b47f5087eda97ac6833b23f"].into(),
                hex!["b4801d80cd15b61d9774bf163c60d2068606f2f00f2ff4ee1f7a720062f2f775"].into(),
                hex!["b23f016ff3adc74468775edc594c887cfdb0636468f6a522ec41a434a5367969"].into(),
                hex!["3025a89a305f6e1a5599b3b9f551f907950c97d8e7614b6c248a26e36e4e7d2a"].into(),

                // authority #8
                hex!["3af49d16869a494380772e47fb2d92f114c3190e291ed655c3bf9b9b5df52e42"].into(),
                hex!["98758865268f1a6f7c1307b6ffe18cb764ce9d57561c97423ac145c8d84a4957"].into(),
                hex!["14d83dc11288148747df1f92d11ad4c5b42dbb12f4aa69f679b36f1f84d41ae0"].into(),
                hex!["6e239d6162484e5c80efcb4762a98e211e11096f5b71bea251ff5732e3c39244"].into(),
                hex!["aa7b10d9bc12874940f92d98f99e7c817c2f86e0630847e3fa124035c6e69a69"].into(),
                hex!["1006efa023a6729f0cb8908387e847c42f064d9fc8fe71f1bd869a18fc675f56"].into(),

                // authority #9
                hex!["603af684fbf8a984af4cc7e147673b576f91da1b3f55e0835a5655a4470e7f22"].into(),
                hex!["b00da5291fc18c973700c7aeab0ca13bdb9f3e48127657c7c9273e75eb7eb27d"].into(),
                hex!["c95b718120a73e30ba70bb2a9d369eeb87ed1f5708f21e66b6cb5d7bcfb8c8f7"].into(),
                hex!["48be9f059c8363935514c7cfe3a4f096a77f101a112539057789c9c4fc54a14d"].into(),
                hex!["805a13b47ac6089a144618a54a0a1f1dd5da9c8bcdf6f54390d882b8d8ff616c"].into(),
                hex!["18e905ae2fbe168ebc14b6682179255ea6410733a0d52e69ed49f3717d73236c"].into(),

                // authority #10
                hex!["322b80c5529a20a1da5702acfb78879211eed2110d687ca001309c1c6d57030f"].into(),
                hex!["48357721e05e42c153e3f33e739b56c4f711762cb45c5463c0b0891bd49fa64b"].into(),
                hex!["f44fcaa91171530462d0d43225354d09c0e64fc9ac7e6bed017279947d6a4785"].into(),
                hex!["0233e16df3c3fbed4dcc5cf47e694ed484373899d712665c39ef4441d2fae040"].into(),
                hex!["2ec48f57ff1730098daf6a65addefce46f22bff0a5b58b098f306f13b01c145d"].into(),
                hex!["886e84efd495efa7b46a8b9a9c45a4e02174ba5b94bdd32381a1bebbdfaf7c77"].into(),
            ],
			true,
		),
        vec![],
        // Telemetry Endpoints
        Some(
            TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Polkadot telemetry url is valid; qed"),
        ),
        // Protocol ID
        Some("dhx-test"),
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
	))
}

pub fn datahighway_mainnet_westlake_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm binary not available".to_string())?;

    let mut properties = Map::new();
    properties.insert("tokenSymbol".into(), "DHX".into());
    properties.insert("tokenDecimals".into(), 18.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"DataHighway Westlake Mainnet",
		// ID
		"westlake",
        ChainType::Live,
		move || mainnet_genesis(
			wasm_binary,
			// Initial NPoS authorities
            vec![
                // authority #1
                (
                    // stash
                    hex!["3c62839f3ce86df3c27c66016b092d15186b23e429db4ad8338501fc219bfa6c"].into(),
                    // cont
                    hex!["42b728d9c752fe87c3e3db40d9a7d02f22b81bc1f0e49c59a5e128b861f87b08"].into(),
                    // gran
                    hex!["dce69d42cf6c256e1ba1595300d72797429dc415f9803e54e822416b6748dfa2"]
                        .unchecked_into(),
                    // babe
                    hex!["ecb52b9c85909f0a171095022351cced6673e1c9a2930087e1c3f37be6c5cd4c"]
                        .unchecked_into(),
                    // imon
                    hex!["ce0b5a11f5447727bf8382426620439b716846b539a0045fcf459299a3237c28"]
                        .unchecked_into(),
                    // audi
                    hex!["9c4471e94b5f8b7741418df3d5e8746970fabe84466cce7346d73fcc7705130b"]
                        .unchecked_into(),
                ),
                // authority #2
                (
                    // stash
                    hex!["628580490f55f5340d8a5e5af85eb78fdc066f6ac0385ccb3d17dc9a8b3e651f"].into(),
                    // cont
                    hex!["e627945747e5a66afd5ff3f383819dfb6dd9633f6c9d52b24b10307cc841d577"].into(),
                    // gran
                    hex!["5e5101464eb9a9d2637a18627632e0817c8592472fc498da76661260337398d9"]
                        .unchecked_into(),
                    // babe
                    hex!["9684a0fea1f44ec1558543bd459967225dfdffe260a670981f0ce09d85223278"]
                        .unchecked_into(),
                    // imon
                    hex!["f28436fb808af1121b3873b72a5b818f4231c5c61dfde2cf0d40cfce57b6f03d"]
                        .unchecked_into(),
                    // audi
                    hex!["2c4d8c20018b86a3ddab3c501f822e83343b35e670b2b425e53130bd30a5e767"]
                        .unchecked_into(),
                ),
                // authority #3
                (
                    // stash
                    hex!["44d784f6d346d337a98e273f683590c661ea20b83aa6c9ac93754e02cee4372e"].into(),
                    // cont
                    hex!["32e28b6f06a728b13d782b007d57ab53d7fe3fa2d5def2c58585d27007d5ea05"].into(),
                    // gran
                    hex!["4b0d281fbe2d89bae3955146f10e79c1e63db6aa0d463dc574abafa79656e859"]
                        .unchecked_into(),
                    // babe
                    hex!["7c863c62d1615d7768507cea4dd7678dc3836691ba89697656f66554400c894f"]
                        .unchecked_into(),
                    // imon
                    hex!["9e6987563f86381f47761ee5ac259e1484b25f5bd5342d6bd975a07cb16c6a6b"]
                        .unchecked_into(),
                    // audi
                    hex!["66d5202a408919593bc85e59c18afd9746a432a503c798b0490b4ca9bd3d0930"]
                        .unchecked_into(),
                ),
                // authority #4
                (
                    // stash
                    hex!["1e7c18d12311c46ca4fd9fba48f11ea81c7c87e992fc5b1abb4903219cc6b429"].into(),
                    // cont
                    hex!["3ee5ed5cdf314660b60c6acb87d51af5f10c5d8ea24dd762df4c5973a1683416"].into(),
                    // gran
                    hex!["d2e23c0445afc9e714a9ad9307255150e26e5661f1f2fa57cbb35666f1ff3bfd"]
                        .unchecked_into(),
                    // babe
                    hex!["c0fe72b9f944baa43663310c6dc213c416aea90b8f5f72a87feee049d66dec50"]
                        .unchecked_into(),
                    // imon
                    hex!["b8916d751735fbb43d1ff679ea017cfdcf9ad7f76cf775c8d5d6e0ffe5454c79"]
                        .unchecked_into(),
                    // audi
                    hex!["f2b840b98ea57ca98f17b38493e29c614bf221e8cff7813c71742efd30ff9f59"]
                        .unchecked_into(),
                ),
                // authority #5
                (
                    // stash
                    hex!["b0e6b234c77cc3d9fde0f3a039779c8d6ae0a66cb95d764acc54667e74f4c800"].into(),
                    // cont
                    hex!["2643aaa4cf95aa0f014c015722e31a356eb8bd17888f74bca1ca56c404445c39"].into(),
                    // gran
                    hex!["683d89df05242920e0fdfac6f854b6c96f2fba7934b845bfb60869eeb21549b1"]
                        .unchecked_into(),
                    // babe
                    hex!["84622ff3f45a810e6445cbba2f39c90fd6c93de763f0246d92dc140ef593df2c"]
                        .unchecked_into(),
                    // imon
                    hex!["a42b23e60c547565c52f768fee92d1ab983456f8b14957dcd59e96edb9e18e34"]
                        .unchecked_into(),
                    // audi
                    hex!["ca0949009a9d8d6b42c04751700efe2372ccf97c4c38bb0237079684d2badc33"]
                        .unchecked_into(),
                ),
                // authority #6
                (
                    // stash
                    hex!["48e771e75097d353d06b5fd469edf8cb53d8c9b8eeeb1aa0f480ff7a42e27f28"].into(),
                    // cont
                    hex!["3e028f22ca42f5feee5344c3319ba97d23019087a124b43a825cb7d35ed7d522"].into(),
                    // gran
                    hex!["215a2ff9562ae7b2f71e07c5976637dfd9dae4c092e18af9828a078ba57c0da0"]
                        .unchecked_into(),
                    // babe
                    hex!["6c2d436d832afc82cce4aac80f8b19a48238f231bf8f30f95ef79173f5773977"]
                        .unchecked_into(),
                    // imon
                    hex!["8ed3a7c2bd72256130d7add011c7fe6ad78942970bcfe6f811bbc5a24da48c2f"]
                        .unchecked_into(),
                    // audi
                    hex!["98d60144aea5209c33cabb647c4de4d93e8ed3e1ff2a5af72a23693a6917262a"]
                        .unchecked_into(),
                ),
                // authority #7
                (
                    // stash
                    hex!["1ae9a95688b6dcf6db83c61789e59352cdf363c5b13d39f37c27f7434439027e"].into(),
                    // cont
                    hex!["76ea25fc43fbdd113efabf6a12ea1f67c2916f9dd15d7c08a020269aa28cd521"].into(),
                    // gran
                    hex!["ef3f7c3d180b63988ab894d9461a8b989d215604e7c5dbb2ce07f51733c680c5"]
                        .unchecked_into(),
                    // babe
                    hex!["86795b955666d426473dff40ce49c1234c58697c032d295b57c91f612ef44876"]
                        .unchecked_into(),
                    // imon
                    hex!["2082036c6ee9a9680e43433b63235317b6cf7c90f88f0dc6b55d3e7ca0899b7c"]
                        .unchecked_into(),
                    // audi
                    hex!["588493c37be8996e7e50dbce15ffb6ab3caad354b4102f12afc5483bd61c2d3a"]
                        .unchecked_into(),
                ),
                // authority #8
                (
                    // stash
                    hex!["ec9ee2c38014483a454cfe108cc062e0ff641fef7c5c10f0a0f797cfdb860708"].into(),
                    // cont
                    hex!["ee323b965d01799f4af213347549ab5e8e533071df69c4d2ed122a354deb930c"].into(),
                    // gran
                    hex!["a50919f31950b902e110f0e455ca2307b021f062c24b087fd94a922be68c1618"]
                        .unchecked_into(),
                    // babe
                    hex!["6e094819c4f3d6bebfd83dfd13b15f57a000d7876d8cf22fd774a593a7304544"]
                        .unchecked_into(),
                    // imon
                    hex!["189000f2b00393cac4ffbfb212ab582c2fdb42c2bea382ab0e9bb5b37d427b03"]
                        .unchecked_into(),
                    // audi
                    hex!["aa04bd5ef43bdcc0090737eae432fd32d1a31cdadbd02babb12d548a280d647f"]
                        .unchecked_into(),
                ),
                // authority #9
                (
                    // stash
                    hex!["746be7c22192aa11d46d3b2bdad13cc77b1bab69ab9eb10799b629dd7ddfbf7e"].into(),
                    // cont
                    hex!["cc517121c11d0135836a62816526bb40d9c9a0ae47f316c646c148a5cff0f200"].into(),
                    // gran
                    hex!["bcd9f49d8a3ce7f0cd71e7effbdfa1aa472f2bd7c66782a71d25be7d98a2c60f"]
                        .unchecked_into(),
                    // babe
                    hex!["98d0045128cc74ec6c6682dd465dd8f5a9c55f0a18d4c2360337c0e727d9b679"]
                        .unchecked_into(),
                    // imon
                    hex!["086c4fbfa5ec20f5bc4d82733f7741ef0f772e2a6d282fd7e6c0e1abf838697b"]
                        .unchecked_into(),
                    // audi
                    hex!["8ecd8e1a9181dc45baae33d09c2c576612f1adf52a294c4e3a5383ef6ebd1c1d"]
                        .unchecked_into(),
                ),
                // authority #10
                (
                    // stash
                    hex!["ecc71d63ef4b80d8feab189a764cd3e759f941062f9b07168bbfcb814da7bf33"].into(),
                    // cont
                    hex!["3c71cfbd77668301af5aefe0c81e3b4ffc1c0f0b07c0d42044922237d574455f"].into(),
                    // gran
                    hex!["55adaf3d56e97313351424ae62742678ead13c5835dc90b7d1a138ba300f8b93"]
                        .unchecked_into(),
                    // babe
                    hex!["9608d194e070706ae1a63a25e522c53bc80205b174956e804be0dcb1a6d78e10"]
                        .unchecked_into(),
                    // imon
                    hex!["1a811ced21ced4fdab5d32105a65c671da35f367937619ab4fbdc3db9542c450"]
                        .unchecked_into(),
                    // audi
                    hex!["029e12aad36b688dda6937ba93fb95b8076d8242865e9adca69b4182ef77f843"]
                        .unchecked_into(),
                ),
            ],
			// Sudo account
            hex!["c201d4551d04a99772d8efe196490a96b4ee5e608ac8e495be9505a99e723069"].into(),
			// Pre-funded accounts
            vec![
                // Endow the Sudo account to cover transaction fees
                hex!["c201d4551d04a99772d8efe196490a96b4ee5e608ac8e495be9505a99e723069"].into(),
                // Endow the Treasury account with the DHX DAO Unlocked Reserves Balance
                hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into(),
                // Endow these accounts with a balance so they may bond as authorities.
                // IMPORTANT: All authorities must be included in the list below so they have
                // an account balance to avoid session error
                // `assertion failed: frame_system::Module::<T>::inc_consumers(&account).is_ok()`

                // authority #1
                // stash
                hex!["3c62839f3ce86df3c27c66016b092d15186b23e429db4ad8338501fc219bfa6c"].into(),
                // cont
                hex!["42b728d9c752fe87c3e3db40d9a7d02f22b81bc1f0e49c59a5e128b861f87b08"].into(),
                // gran
                hex!["dce69d42cf6c256e1ba1595300d72797429dc415f9803e54e822416b6748dfa2"].into(),
                // babe
                hex!["ecb52b9c85909f0a171095022351cced6673e1c9a2930087e1c3f37be6c5cd4c"].into(),
                // imon
                hex!["ce0b5a11f5447727bf8382426620439b716846b539a0045fcf459299a3237c28"].into(),
                // audi
                hex!["9c4471e94b5f8b7741418df3d5e8746970fabe84466cce7346d73fcc7705130b"].into(),

                // authority #2
                // stash
                hex!["628580490f55f5340d8a5e5af85eb78fdc066f6ac0385ccb3d17dc9a8b3e651f"].into(),
                // cont
                hex!["e627945747e5a66afd5ff3f383819dfb6dd9633f6c9d52b24b10307cc841d577"].into(),
                // gran
                hex!["5e5101464eb9a9d2637a18627632e0817c8592472fc498da76661260337398d9"].into(),
                // babe
                hex!["9684a0fea1f44ec1558543bd459967225dfdffe260a670981f0ce09d85223278"].into(),
                // imon
                hex!["f28436fb808af1121b3873b72a5b818f4231c5c61dfde2cf0d40cfce57b6f03d"].into(),
                // audi
                hex!["2c4d8c20018b86a3ddab3c501f822e83343b35e670b2b425e53130bd30a5e767"].into(),

                // authority #3
                // stash
                hex!["44d784f6d346d337a98e273f683590c661ea20b83aa6c9ac93754e02cee4372e"].into(),
                // cont
                hex!["32e28b6f06a728b13d782b007d57ab53d7fe3fa2d5def2c58585d27007d5ea05"].into(),
                // gran
                hex!["4b0d281fbe2d89bae3955146f10e79c1e63db6aa0d463dc574abafa79656e859"].into(),
                // babe
                hex!["7c863c62d1615d7768507cea4dd7678dc3836691ba89697656f66554400c894f"].into(),
                // imon
                hex!["9e6987563f86381f47761ee5ac259e1484b25f5bd5342d6bd975a07cb16c6a6b"].into(),
                // audi
                hex!["66d5202a408919593bc85e59c18afd9746a432a503c798b0490b4ca9bd3d0930"].into(),

                // authority #4
                // stash
                hex!["1e7c18d12311c46ca4fd9fba48f11ea81c7c87e992fc5b1abb4903219cc6b429"].into(),
                // cont
                hex!["3ee5ed5cdf314660b60c6acb87d51af5f10c5d8ea24dd762df4c5973a1683416"].into(),
                // gran
                hex!["d2e23c0445afc9e714a9ad9307255150e26e5661f1f2fa57cbb35666f1ff3bfd"].into(),
                // babe
                hex!["c0fe72b9f944baa43663310c6dc213c416aea90b8f5f72a87feee049d66dec50"].into(),
                // imon
                hex!["b8916d751735fbb43d1ff679ea017cfdcf9ad7f76cf775c8d5d6e0ffe5454c79"].into(),
                // audi
                hex!["f2b840b98ea57ca98f17b38493e29c614bf221e8cff7813c71742efd30ff9f59"].into(),

                // authority #5
                // stash
                hex!["b0e6b234c77cc3d9fde0f3a039779c8d6ae0a66cb95d764acc54667e74f4c800"].into(),
                // cont
                hex!["2643aaa4cf95aa0f014c015722e31a356eb8bd17888f74bca1ca56c404445c39"].into(),
                // gran
                hex!["683d89df05242920e0fdfac6f854b6c96f2fba7934b845bfb60869eeb21549b1"].into(),
                // babe
                hex!["84622ff3f45a810e6445cbba2f39c90fd6c93de763f0246d92dc140ef593df2c"].into(),
                // imon
                hex!["a42b23e60c547565c52f768fee92d1ab983456f8b14957dcd59e96edb9e18e34"].into(),
                // audi
                hex!["ca0949009a9d8d6b42c04751700efe2372ccf97c4c38bb0237079684d2badc33"].into(),

                // authority #6
                // stash
                hex!["48e771e75097d353d06b5fd469edf8cb53d8c9b8eeeb1aa0f480ff7a42e27f28"].into(),
                // cont
                hex!["3e028f22ca42f5feee5344c3319ba97d23019087a124b43a825cb7d35ed7d522"].into(),
                // gran
                hex!["215a2ff9562ae7b2f71e07c5976637dfd9dae4c092e18af9828a078ba57c0da0"].into(),
                // babe
                hex!["6c2d436d832afc82cce4aac80f8b19a48238f231bf8f30f95ef79173f5773977"].into(),
                // imon
                hex!["8ed3a7c2bd72256130d7add011c7fe6ad78942970bcfe6f811bbc5a24da48c2f"].into(),
                // audi
                hex!["98d60144aea5209c33cabb647c4de4d93e8ed3e1ff2a5af72a23693a6917262a"].into(),

                // authority #7
                // stash
                hex!["1ae9a95688b6dcf6db83c61789e59352cdf363c5b13d39f37c27f7434439027e"].into(),
                // cont
                hex!["76ea25fc43fbdd113efabf6a12ea1f67c2916f9dd15d7c08a020269aa28cd521"].into(),
                // gran
                hex!["ef3f7c3d180b63988ab894d9461a8b989d215604e7c5dbb2ce07f51733c680c5"].into(),
                // babe
                hex!["86795b955666d426473dff40ce49c1234c58697c032d295b57c91f612ef44876"].into(),
                // imon
                hex!["2082036c6ee9a9680e43433b63235317b6cf7c90f88f0dc6b55d3e7ca0899b7c"].into(),
                // audi
                hex!["588493c37be8996e7e50dbce15ffb6ab3caad354b4102f12afc5483bd61c2d3a"].into(),

                // authority #8
                // stash
                hex!["ec9ee2c38014483a454cfe108cc062e0ff641fef7c5c10f0a0f797cfdb860708"].into(),
                // cont
                hex!["ee323b965d01799f4af213347549ab5e8e533071df69c4d2ed122a354deb930c"].into(),
                // gran
                hex!["a50919f31950b902e110f0e455ca2307b021f062c24b087fd94a922be68c1618"].into(),
                // babe
                hex!["6e094819c4f3d6bebfd83dfd13b15f57a000d7876d8cf22fd774a593a7304544"].into(),
                // imon
                hex!["189000f2b00393cac4ffbfb212ab582c2fdb42c2bea382ab0e9bb5b37d427b03"].into(),
                // audi
                hex!["aa04bd5ef43bdcc0090737eae432fd32d1a31cdadbd02babb12d548a280d647f"].into(),

                // authority #9
                // stash
                hex!["746be7c22192aa11d46d3b2bdad13cc77b1bab69ab9eb10799b629dd7ddfbf7e"].into(),
                // cont
                hex!["cc517121c11d0135836a62816526bb40d9c9a0ae47f316c646c148a5cff0f200"].into(),
                // gran
                hex!["bcd9f49d8a3ce7f0cd71e7effbdfa1aa472f2bd7c66782a71d25be7d98a2c60f"].into(),
                // babe
                hex!["98d0045128cc74ec6c6682dd465dd8f5a9c55f0a18d4c2360337c0e727d9b679"].into(),
                // imon
                hex!["086c4fbfa5ec20f5bc4d82733f7741ef0f772e2a6d282fd7e6c0e1abf838697b"].into(),
                // audi
                hex!["8ecd8e1a9181dc45baae33d09c2c576612f1adf52a294c4e3a5383ef6ebd1c1d"].into(),

                // authority #10
                // stash
                hex!["ecc71d63ef4b80d8feab189a764cd3e759f941062f9b07168bbfcb814da7bf33"].into(),
                // cont
                hex!["3c71cfbd77668301af5aefe0c81e3b4ffc1c0f0b07c0d42044922237d574455f"].into(),
                // gran
                hex!["55adaf3d56e97313351424ae62742678ead13c5835dc90b7d1a138ba300f8b93"].into(),
                // babe
                hex!["9608d194e070706ae1a63a25e522c53bc80205b174956e804be0dcb1a6d78e10"].into(),
                // imon
                hex!["1a811ced21ced4fdab5d32105a65c671da35f367937619ab4fbdc3db9542c450"].into(),
                // audi
                hex!["029e12aad36b688dda6937ba93fb95b8076d8242865e9adca69b4182ef77f843"].into(),
            ],
			true,
		),
        vec![
            // authority #1
            "/ip4/3.127.123.230/tcp/30333/p2p/12D3KooWPSVWEpuNPKE6EJBAMQQRCrKG4RTfyyabFRjT4xqMkuH5"
                .parse()
                .unwrap(),
            // authority #2
            "/ip4/3.65.196.4/tcp/30333/p2p/12D3KooWPZqAuWSez5uomot7GZvpuRQK198zqLYrLLZt5W7bvqPb"
                .parse()
                .unwrap(),
            // authority #3
            "/ip4/3.123.21.153/tcp/30333/p2p/12D3KooWAjdURBpSsRVWbvnGRbsqykvueM6Vuoe4x7MhV6cxTtje"
                .parse()
                .unwrap(),
            // authority #4
            "/ip4/18.184.76.132/tcp/30333/p2p/12D3KooWCWZc5L6ypCFcvDdGeGwsw9Mo4nniCwiVuU5MB6ApA4ZT"
                .parse()
                .unwrap(),
            // authority #5
            "/ip4/3.124.189.68/tcp/30333/p2p/12D3KooWJ1F4BsNgeaVkZVPw2kRhHxAtJuUqeEik2R7dv9ttgPcv"
                .parse()
                .unwrap(),
            // authority #6
            "/ip4/104.236.197.177/tcp/30333/p2p/12D3KooWGgVUU6V4MNqhw4Fcbb7u5abEdD2QgLgx3TKmVXovcUft"
                .parse()
                .unwrap(),
            // authority #7
            "/ip4/104.236.197.174/tcp/30333/p2p/12D3KooWFGzcJWw7a1q1Sgn8qiLgnQ8UBy3DZqP33ZhrEXabgpm5"
                .parse()
                .unwrap(),
            // authority #8
            "/ip4/104.236.197.180/tcp/30333/p2p/12D3KooWFz44eN1nhVEAeq4x7Z4Hdd6GA9dhpc9LSXDmJA4aqfd6"
                .parse()
                .unwrap(),
            // authority #9
            "/ip4/104.236.197.172/tcp/30333/p2p/12D3KooWJjuKnSjF3fgrsAPv1b3VZH2nd7qzcVmh9imPNJTbEtSV"
                .parse()
                .unwrap(),
            // authority #10
            "/ip4/104.236.197.182/tcp/30333/p2p/12D3KooWST5nKEAFNXnLLQvjnAX88D99yF8Y1XVebSybVgcLDJzz"
                .parse()
                .unwrap(),
        ],
        // Telemetry Endpoints
        Some(
            TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Polkadot telemetry url is valid; qed"),
        ),
        // Protocol ID
        Some("dhx-mainnet"),
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
	))
}

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

// Testnet

// in testnet total supply should be 100m, with 30m (30%) going to DHX DAO unlocked reserves, and the remaining
// 70m split between the initial accounts other than the reserves
const TESTNET_INITIAL_ENDOWMENT: u128 = 10_000_000_000_000_000_000_u128; // 10 DHX
const TESTNET_INITIAL_DHX_DAO_TREASURY_UNLOCKED_RESERVES_BALANCE: u128 = 30_000_000_000_000_000_000_000_000_u128; // 30M DHX
const TESTNET_INITIAL_STASH: u128 = MAINNET_INITIAL_ENDOWMENT / 10; // 1 DHX

// Mainnet
const MAINNET_INITIAL_ENDOWMENT: u128 = 10_000_000_000_000_000_000_u128; // 10 DHX
const MAINNET_INITIAL_DHX_DAO_TREASURY_UNLOCKED_RESERVES_BALANCE: u128 = 30_000_000_000_000_000_000_000_000_u128; // 30M DHX
const MAINNET_INITIAL_STASH: u128 = MAINNET_INITIAL_ENDOWMENT / 10; // 1 DHX

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool, // No println
) -> GenesisConfig {
    let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
        frame_system: Some(SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| {
                    // Insert Public key (hex) of the account without the 0x prefix below
                    if x == UncheckedFrom::unchecked_from(
                        hex!("6d6f646c70792f74727372790000000000000000000000000000000000000000").into(),
                    ) {
                        // If we use println, then the top of the chain specification file that gets
                        // generated contains the println, and then we have to remove the println from
                        // the top of that file to generate the "raw" chain definition
                        // println!("endowed_account treasury {:?}", x.clone());
                        return (x, TESTNET_INITIAL_DHX_DAO_TREASURY_UNLOCKED_RESERVES_BALANCE);
                    } else {
                        // println!("endowed_account {:?}", x.clone());
                        return (x, TESTNET_INITIAL_ENDOWMENT);
                    }
                })
            .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: endowed_accounts.iter().enumerate().map(|(index, x)| (index as u32, (*x).clone())).collect(),
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone())))
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), TESTNET_INITIAL_STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, TESTNET_INITIAL_STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(Default::default()),
        pallet_collective_Instance2: Some(Default::default()),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key.clone(),
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig {
            keys: vec![],
        }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(TechnicalMembershipConfig {
            members: vec![root_key.clone()],
            phantom: Default::default(),
        }),
        pallet_treasury: Some(TreasuryConfig::default()),
	}
}

/// Configure initial storage state for FRAME modules.
fn mainnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool, // No println
) -> GenesisConfig {
    let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
        frame_system: Some(SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| {
                    // Insert Public key (hex) of the account without the 0x prefix below
                    if x == UncheckedFrom::unchecked_from(
                        hex!("6d6f646c70792f74727372790000000000000000000000000000000000000000").into(),
                    ) {
                        // println!("endowed_account treasury {:?}", x.clone());
                        return (x, MAINNET_INITIAL_DHX_DAO_TREASURY_UNLOCKED_RESERVES_BALANCE);
                    } else {
                        // println!("endowed_account {:?}", x.clone());
                        return (x, MAINNET_INITIAL_ENDOWMENT);
                    }
                })
            .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: endowed_accounts.iter().enumerate().map(|(index, x)| (index as u32, (*x).clone())).collect(),
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone())))
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), MAINNET_INITIAL_STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, MAINNET_INITIAL_STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(Default::default()),
        pallet_collective_Instance2: Some(Default::default()),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key.clone(),
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig {
            keys: vec![],
        }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(TechnicalMembershipConfig {
            members: vec![root_key.clone()],
            phantom: Default::default(),
        }),
        pallet_treasury: Some(TreasuryConfig::default()),
	}
}
