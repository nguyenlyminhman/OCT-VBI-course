#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

/*
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
*/
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::*, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn member)]
	//Member storage
	pub type Room<T: Config > = StorageValue<_, Vec<T::AccountId>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddToRoom(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		CustomerExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add(origin: OriginFor<T>) -> DispatchResult {
			let _customer = ensure_signed(origin)?;

			//Check member exist or not , then write into storage
			match Room::<T>::get() {
				None => {
					let mut customers = Vec::new();
					customers.push(&_customer);
					<Room<T>>::put(customers);
				},
				Some(v) => {
					let mut customers = v;
					if !customers.contains(&_customer) {
						customers.push(_customer.clone());
						<Room<T>>::put(customers);
						Self::deposit_event(Event::AddToRoom(_customer));
					}
					else {
						return Err(Error::<T>::CustomerExist)?;
					}

				}
			}
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}
}