// Copyright (c) 2023 Encointer Association
// This file is part of Encointer
//
// Encointer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Encointer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Encointer.  If not, see <http://www.gnu.org/licenses/>.

// the following are temporary local migration fixes to solve inconsistencies caused by not
// migrating Storage at the time of migrating runtime code

pub mod scheduler {
	// this is necessary because migrations from v0 to v3 are no longer available in the scheduler
	// pallet code and migrating is only possible from v3. The strategy here is to empty the agenda
	// (has been empty since genesis)
	use frame_support::traits::OnRuntimeUpgrade;
	use frame_system::pallet_prelude::BlockNumberFor;
	use pallet_scheduler::*;
	use sp_std::vec::Vec;

	/// The log target.
	const TARGET: &'static str = "runtime::fix::scheduler::migration";

	pub mod v1 {
		use super::*;
		use frame_support::{pallet_prelude::*, traits::schedule};

		#[cfg_attr(any(feature = "std", test), derive(PartialEq, Eq))]
		#[derive(Clone, RuntimeDebug, Encode, Decode)]
		pub(crate) struct ScheduledV1<Call, BlockNumber> {
			maybe_id: Option<Vec<u8>>,
			priority: schedule::Priority,
			call: Call,
			maybe_periodic: Option<schedule::Period<BlockNumber>>,
		}

		#[frame_support::storage_alias]
		pub(crate) type Agenda<T: Config> = StorageMap<
			Pallet<T>,
			Twox64Concat,
			BlockNumberFor<T>,
			Vec<Option<ScheduledV1<<T as Config>::RuntimeCall, BlockNumberFor<T>>>>,
			ValueQuery,
		>;

		#[frame_support::storage_alias]
		pub(crate) type Lookup<T: Config> =
			StorageMap<Pallet<T>, Twox64Concat, Vec<u8>, TaskAddress<BlockNumberFor<T>>>;
	}

	pub mod v3 {
		use super::*;
		use frame_support::pallet_prelude::*;

		#[frame_support::storage_alias]
		pub(crate) type Agenda<T: Config> = StorageMap<
			Pallet<T>,
			Twox64Concat,
			BlockNumberFor<T>,
			Vec<Option<ScheduledV3Of<T>>>,
			ValueQuery,
		>;

		#[frame_support::storage_alias]
		pub(crate) type Lookup<T: Config> =
			StorageMap<Pallet<T>, Twox64Concat, Vec<u8>, TaskAddress<BlockNumberFor<T>>>;
	}

	pub mod v4 {
		use super::*;
		use frame_support::pallet_prelude::*;

		#[frame_support::storage_alias]
		pub type Agenda<T: Config> = StorageMap<
			Pallet<T>,
			Twox64Concat,
			BlockNumberFor<T>,
			BoundedVec<
				Option<ScheduledOf<T>>,
				<T as pallet_scheduler::Config>::MaxScheduledPerBlock,
			>,
			ValueQuery,
		>;

		pub(crate) type TaskName = [u8; 32];

		#[frame_support::storage_alias]
		pub(crate) type Lookup<T: Config> =
			StorageMap<Pallet<T>, Twox64Concat, TaskName, TaskAddress<BlockNumberFor<T>>>;

		/// Migrate the scheduler pallet from V0 to V4 without changing storage. the only active schedule has been submitted already in V4
		pub struct MigrateToV4<T>(sp_std::marker::PhantomData<T>);

		impl<T: Config> OnRuntimeUpgrade for MigrateToV4<T> {
			#[cfg(feature = "try-runtime")]
			fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
				let agendas = v1::Agenda::<T>::iter_keys().count() as u32;
				let lookups = v1::Lookup::<T>::iter_keys().count() as u32;
				log::info!(target: TARGET, "agendas present which will be left untouched: {}/{}...", agendas, lookups);
				Ok((agendas, lookups).encode())
			}

			fn on_runtime_upgrade() -> Weight {
				let onchain_version = Pallet::<T>::on_chain_storage_version();
				if onchain_version >= 3 {
					log::warn!(
						target: TARGET,
						"skipping v0 to v4 migration: executed on wrong storage version.\
					Expected version < 3, found {:?}",
						onchain_version,
					);
					return T::DbWeight::get().reads(1)
				}
				log::info!(target: TARGET, "migrating from {:?} to 4", onchain_version);
				StorageVersion::new(4).put::<Pallet<T>>();

				T::DbWeight::get().reads_writes(1, 1)
			}

			#[cfg(feature = "try-runtime")]
			fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
				ensure!(StorageVersion::get::<Pallet<T>>() == 4, "Must upgrade");

				let agendas = Agenda::<T>::iter_keys().count() as u32;
				let lookups = Lookup::<T>::iter_keys().count() as u32;
				log::info!(target: TARGET, "agendas present a posteriori: {}/{}...", agendas, lookups);
				Ok(())
			}
		}
	}
}

pub mod collective {
	// this is necessary because migrations from v0 to v3 are no longer available in the scheduler
	// pallet code and migrating is only possible from v3. The strategy here is to empty the agenda
	// (has been empty since genesis)
	use frame_support::traits::OnRuntimeUpgrade;
	use pallet_collective::*;
	use sp_std::vec::Vec;

	/// The log target.
	const TARGET: &'static str = "runtime::fix::collective::migration";

	pub mod v4 {
		use super::*;
		use frame_support::pallet_prelude::*;

		/// Migrate the scheduler pallet from V0 to V4 without changing storage. the only active schedule has been submitted already in V4
		pub struct MigrateToV4<T>(sp_std::marker::PhantomData<T>);

		impl<T: Config> OnRuntimeUpgrade for MigrateToV4<T> {
			#[cfg(feature = "try-runtime")]
			fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
				Ok((0u32).encode())
			}

			fn on_runtime_upgrade() -> Weight {
				let onchain_version = Pallet::<T>::on_chain_storage_version();
				if onchain_version >= 3 {
					log::warn!(
						target: TARGET,
						"skipping v0 to v4 migration: executed on wrong storage version.\
					Expected version < 3, found {:?}",
						onchain_version,
					);
					return T::DbWeight::get().reads(1)
				}
				log::info!(target: TARGET, "migrating from {:?} to 4", onchain_version);
				StorageVersion::new(4).put::<Pallet<T>>();

				T::DbWeight::get().reads_writes(1, 1)
			}

			#[cfg(feature = "try-runtime")]
			fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
				ensure!(StorageVersion::get::<Pallet<T>>() == 4, "Must upgrade");
				Ok(())
			}
		}
	}
}

//PolkadotXcm pallet
pub mod xcm {
	// this is necessary because migrations from v0 to v3 are no longer available in the scheduler
	// pallet code and migrating is only possible from v3. The strategy here is to empty the agenda
	// (has been empty since genesis)
	use frame_support::traits::OnRuntimeUpgrade;
	use pallet_xcm::*;
	use sp_std::vec::Vec;

	/// The log target.
	const TARGET: &'static str = "runtime::fix::xcm::migration";

	pub mod v1 {
		use super::*;
		use frame_support::pallet_prelude::*;
		use xcm::{prelude::XcmVersion, v3::QueryId, VersionedMultiLocation};

		#[frame_support::storage_alias]
		pub(super) type VersionNotifyTargets<T: Config> = StorageDoubleMap<
			Pallet<T>,
			Twox64Concat,
			XcmVersion,
			Blake2_128Concat,
			VersionedMultiLocation,
			(QueryId, Weight, XcmVersion),
			OptionQuery,
		>;

		pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);

		impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
			#[cfg(feature = "try-runtime")]
			fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
				let targets = VersionNotifyTargets::<T>::iter_prefix_values(3).count() as u32;
				log::info!(target: TARGET, "found {} VersionNotifyTargets", targets);
				Ok(targets.encode())
			}

			fn on_runtime_upgrade() -> Weight {
				let onchain_version = Pallet::<T>::on_chain_storage_version();
				if onchain_version > 0 {
					log::warn!(
						target: TARGET,
						"skipping v0 to v1 migration: executed on wrong storage version.\
					Expected version 0, found {:?}",
						onchain_version,
					);
					return T::DbWeight::get().reads(1)
				}
				log::info!(target: TARGET, "migrating from {:?} to 1", onchain_version);
				StorageVersion::new(1).put::<Pallet<T>>();

				T::DbWeight::get().reads_writes(1, 1)
			}

			#[cfg(feature = "try-runtime")]
			fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
				ensure!(StorageVersion::get::<Pallet<T>>() == 1, "Must upgrade");
				let old_targets: u32 = Decode::decode(&mut &state[..])
					.expect("pre_upgrade provides a valid state; qed");
				let targets = VersionNotifyTargets::<T>::iter_prefix_values(3);
				assert_eq!(
					old_targets,
					targets.count() as u32,
					"must preserve all targets and be able to decode storage"
				);
				Ok(())
			}
		}
	}
}
