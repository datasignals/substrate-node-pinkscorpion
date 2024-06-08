
// #![cfg_attr(not(feature = "std"), no_std)]


// pub use pallet::*;


// #[frame_support::pallet]
// pub mod pallet {
	
// 	use super::*;
// 	use frame_support::pallet_prelude::*;
// 	use frame_system::pallet_prelude::*;
// 	use scale_info::prelude::vec::Vec;

	
// 	#[pallet::pallet]
// 	pub struct Pallet<T>(_);

	
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config {
// 		/// The overarching runtime event type.
// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
// 	}

// 	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq, Eq)]
// 	pub struct FSEvent {
// 		pub creationtime: [u8; 64],
//     	pub filepath: [u8; 256],
//     	pub eventkey: [u8; 128],
// 	}

// 	#[pallet::storage]
// 	#[pallet::getter(fn info)]
// 	pub(super) type DisReAssembly<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, FSEvent, OptionQuery>;
// 	//pub(super) type DisReAssembly<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u64, FSEvent>;


// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(super) fn deposit_event)]
// 	pub enum Event<T: Config> {
		
// 		// /// An event indicating a file has been disassembled.
//         // FileDisassembled(T::AccountId, Vec<u8>, Vec<u8>, Vec<u8>), // (account, creation_time, file_path, event_key)
//         // /// An event indicating a file has been reassembled.
//         // FileReassembled(T::AccountId, Vec<u8>, Vec<u8>, Vec<u8>),  // (account, creation_time, file_path, event_key)
// 		/// An event indicating a file has been disassembled.
		
// 		FileDisassembled { who: T::AccountId, event: FSEvent },
		
// 		//FileDisassembled { who: T::AccountId, creation_time: Vec<u8>, file_path: Vec<u8>, event_key: Vec<u8> },
// 		/// An event indicating a file has been reassembled.
// 		//FileReassembled { who: T::AccountId, creation_time: Vec<u8>, file_path: Vec<u8>, event_key: Vec<u8> },
// 		FileReassembled { who: T::AccountId, event: FSEvent },
// 	}

	
// 	#[pallet::error]
// 	pub enum Error<T> {
// 		// /// The value retrieved was `None` as no value was previously set.
// 		// NoneValue,
// 		// /// There was an attempt to increment the value in storage over `u32::MAX`.
// 		// StorageOverflow,

// 		CreationTimeTooLong,
//     	FilePathTooLong,
//     	EventKeyTooLong,
// 	}


	
	
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {

// 		#[pallet::call_index(0)]
//         #[pallet::weight(Weight::from_parts(10_000, 0)+ T::DbWeight::get().writes(1))]
//         // pub fn file_disassembled(origin: OriginFor<T>, creation_time: Vec<u8>, file_path: Vec<u8>, event_key: Vec<u8>) -> DispatchResult {
//         //     let sender = ensure_signed(origin)?;
//         //     // Emit event for file disassembly
//         //     Self::deposit_event(Event::FileDisassembled(sender, creation_time, file_path, event_key));
//         //     Ok(())
//         // }

// 		pub fn file_disassembled(
// 			origin: OriginFor<T>,
// 			creation_time: Vec<u8>,
// 			file_path: Vec<u8>,
// 			event_key: Vec<u8>,
// 		) -> DispatchResult {
// 			let sender = ensure_signed(origin)?;
// 			// Validate input lengths
// 			ensure!(creation_time.len() <= 64, Error::<T>::CreationTimeTooLong);
// 			ensure!(file_path.len() <= 256, Error::<T>::FilePathTooLong);
// 			ensure!(event_key.len() <= 128, Error::<T>::EventKeyTooLong);

// 			// Create FSEvent instance
// 			let event = FSEvent {
// 				creationtime: {
// 					let mut arr = [0u8; 64];
// 					arr[..creation_time.len()].copy_from_slice(&creation_time);
// 					arr
// 				},
// 				filepath: {
// 					let mut arr = [0u8; 256];
// 					arr[..file_path.len()].copy_from_slice(&file_path);
// 					arr
// 				},
// 				eventkey: {
// 					let mut arr = [0u8; 128];
// 					arr[..event_key.len()].copy_from_slice(&event_key);
// 					arr
// 				},
// 			};

			
// 			// Store the event in storage
// 			<DisReAssembly<T>>::insert(&sender, &event);

// 			// Emit event for file disassembly
// 			Self::deposit_event(Event::<T>::FileDisassembled { 
// 				who: sender.clone(), 
// 				event: event.clone(),
// 			});

// 			Ok(())  // This is correctly returning a DispatchResult

// 		}

//         // #[pallet::weight(10_000, T::DbWeight::get().writes(1))]
//         #[pallet::call_index(1)]
//         #[pallet::weight(Weight::from_parts(10_000, 0)+ T::DbWeight::get().writes(1))]
//         // pub fn file_reassembled(origin: OriginFor<T>, creation_time: Vec<u8>, file_path: Vec<u8>, event_key: Vec<u8>) -> DispatchResult {
//         //     let sender = ensure_signed(origin)?;
//         //     // Emit event for file reassembly
//         //     Self::deposit_event(Event::FileReassembled(sender, creation_time, file_path, event_key));
//         //     Ok(())
//         // }
		
// 		pub fn file_reassembled(
// 			origin: OriginFor<T>,
// 			creation_time: Vec<u8>,
// 			file_path: Vec<u8>,
// 			event_key: Vec<u8>,
// 		) -> DispatchResult {
// 			let sender = ensure_signed(origin)?;
// 			// Validate input lengths
// 			ensure!(creation_time.len() <= 64, Error::<T>::CreationTimeTooLong);
// 			ensure!(file_path.len() <= 256, Error::<T>::FilePathTooLong);
// 			ensure!(event_key.len() <= 128, Error::<T>::EventKeyTooLong);

// 			// Create FSEvent instance
// 			let event = FSEvent {
// 				creationtime: {
// 					let mut arr = [0u8; 64];
// 					arr[..creation_time.len()].copy_from_slice(&creation_time);
// 					arr
// 				},
// 				filepath: {
// 					let mut arr = [0u8; 256];
// 					arr[..file_path.len()].copy_from_slice(&file_path);
// 					arr
// 				},
// 				eventkey: {
// 					let mut arr = [0u8; 128];
// 					arr[..event_key.len()].copy_from_slice(&event_key);
// 					arr
// 				},
// 			};

			

// 			// Store the event in storage
// 			<DisReAssembly<T>>::insert(&sender, &event);

			
// 			// Emit event for file reassembly
// 			Self::deposit_event(Event::<T>::FileReassembled {
// 				who: sender.clone(), 
// 				event: event.clone(),
// 			});

// 			Ok(())
// 		}
// 	}
// }
