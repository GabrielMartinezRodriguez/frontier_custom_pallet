#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	parameter_types,
	traits::{Currency, Hooks},
};
pub use pallet::*;

use sp_arithmetic::traits::SaturatedConversion;
use sp_core::U256;

parameter_types! {
	pub  InitialValue: U256 = U256::from(1_000_000_000);
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	pub(super) type CustomNumber<T: Config> = StorageValue<_, U256, ValueQuery, InitialValue>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(50_000_000)]
		pub fn give_me_tokens(origin: OriginFor<T>, amount: u128) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			T::Currency::deposit_creating(&who, amount.saturated_into());
			Ok(().into())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: T::BlockNumber) {
			if _n % (100 as u32).into() == (0 as u32).into() {
				<CustomNumber<T>>::mutate(|x| *x = x.saturating_add(U256::from(1)));

				Self::deposit_event(Event::CustomNumberChanged {
					current_value: <CustomNumber<T>>::get(),
				});
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Ethereum events from contracts.
		CustomNumberChanged { current_value: U256 },
	}
}
