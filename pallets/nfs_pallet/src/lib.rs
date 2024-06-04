#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);


    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        //SomethingStored(u32, T::AccountId),
        
        /// An event indicating a file has been disassembled.
        FileDisassembled(T::AccountId, Vec<u8>, Vec<u8>, Vec<u8>), // (account, creation_time, file_path, event_key)
        /// An event indicating a file has been reassembled.
        FileReassembled(T::AccountId, Vec<u8>, Vec<u8>, Vec<u8>),  // (account, creation_time, file_path, event_key)
    }

    

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // #[pallet::weight((10_000, T::DbWeight::get().writes(1)))]
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0)+ T::DbWeight::get().writes(1))]
        pub fn file_disassembled(origin: OriginFor<T>, creation_time: Vec<u8>, file_path: Vec<u8>, event_key: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // Emit event for file disassembly
            Self::deposit_event(Event::FileDisassembled(sender, creation_time, file_path, event_key));
            Ok(())
        }

        // #[pallet::weight(10_000, T::DbWeight::get().writes(1))]
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0)+ T::DbWeight::get().writes(1))]
        pub fn file_reassembled(origin: OriginFor<T>, creation_time: Vec<u8>, file_path: Vec<u8>, event_key: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // Emit event for file reassembly
            Self::deposit_event(Event::FileReassembled(sender, creation_time, file_path, event_key));
            Ok(())
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;
}
