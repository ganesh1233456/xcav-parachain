use super::*;

use crate as pallet_xcavate_staking;
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstBool, ConstU16, ConstU64, Nothing},
};
use sp_core::{ConstU32, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use pallet_contracts::Schedule;

use pallet_transaction_payment::CurrencyAdapter;

use frame_support::traits::ConstU8;

use frame_support::weights::IdentityFee;

use pallet_community_loan_pool::SubstrateWeight;

use frame_support::sp_runtime::Permill;

use pallet_transaction_payment::{ConstFeeMultiplier, Multiplier};

use sp_runtime::traits::One;

use frame_support::PalletId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u32;

pub type BlockNumber = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DAVE: AccountId = 4;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances,
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		CommunityLoanPool: pallet_community_loan_pool,
		Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Randomness: pallet_insecure_randomness_collective_flip::{Pallet, Storage},
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
		XcavateStaking: pallet_xcavate_staking,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u32>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = u32;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU32<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

impl pallet_uniques::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u32>>;
	type ForceOrigin = frame_system::EnsureRoot<u32>;
	type Locker = ();
	type CollectionDeposit = ConstU32<2>;
	type ItemDeposit = ConstU32<1>;
	type MetadataDepositBase = ConstU32<1>;
	type AttributeDepositBase = ConstU32<1>;
	type DepositPerByte = ConstU32<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<u32>;
	type LengthToFee = IdentityFee<u32>;
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

parameter_types! {
	pub const ContractDeposit: u64 = 16;
	pub const DeletionQueueDepth: u32 = 1024;
	pub MySchedule: Schedule<Test> = <Schedule<Test>>::default();
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

impl pallet_contracts::Config for Test {
	type Time = Timestamp;
	type Randomness = Randomness;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type CallFilter = Nothing;
	type DepositPerItem = ();
	type DepositPerByte = ();
	type CallStack = [pallet_contracts::Frame<Self>; 5];
	type WeightPrice = ();
	type WeightInfo = ();
	type ChainExtension = ();
	type DeletionQueueDepth = ();
	type DeletionWeightLimit = ();
	type Schedule = MySchedule;
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type UnsafeUnstableInterface = ConstBool<false>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const CommunityLoanPalletIdPalletId: PalletId = PalletId(*b"py/cmmty");
	pub const MaxLoans: u32 = 10000;
	pub const VotingTime: BlockNumber = 20;
	pub const MaximumCommitteeMembers: u32 = 10;
}

impl pallet_community_loan_pool::Config for Test {
	type PalletId = CommunityLoanPalletIdPalletId;
	type Currency = Balances;
	type ApproveOrigin = frame_system::EnsureRoot<u32>;
	type RejectOrigin = frame_system::EnsureRoot<u32>;
	type CommitteeOrigin = frame_system::EnsureRoot<u32>;
	type RuntimeEvent = RuntimeEvent;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ConstU32<10000>;
	type ProposalBondMaximum = ();
	type OnSlash = ();
	type MaxOngoingLoans = MaxLoans;
	type TimeProvider = Timestamp;
	type WeightInfo = SubstrateWeight<Test>;
	/* #[cfg(feature = "runtime-benchmarks")]
	type Helper = NftHelper; */
	type VotingTime = VotingTime;
	type MaxCommitteeMembers = MaximumCommitteeMembers;
}

parameter_types! {
	pub const MaxStaker: u32 = 10000;
}

impl pallet_xcavate_staking::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MinimumRemainingAmount = ConstU32<100>;
	type MaxStakers = MaxStaker;
	type TimeProvider = Timestamp;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut test = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(ALICE, 20_000_000), (BOB, 15_000), (CHARLIE, 150_000), (DAVE, 5_000)],
	}
	.assimilate_storage(&mut test)
	.unwrap();

	test.into()
}
