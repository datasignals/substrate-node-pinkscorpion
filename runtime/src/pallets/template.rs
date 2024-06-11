use super::types::*;
#[frame_support::pallet]
pub mod pallet_template {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;

    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        FileDisassembled { who: T::AccountId, event: FSEvent },
        FileReassembled { who: T::AccountId, event: FSEvent },
		DisassemblyRequested { who: T::AccountId, event: FSEvent },
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Define your helper functions here as methods of the Pallet struct
    impl<T: Config> Pallet<T> {
        fn fetch_disassembly_requests() -> Vec<Event<T>> {
            // Fetch requests from storage or off-chain storage
            vec![]
        }

        fn process_disassembly_request(request:  Event<T>) -> Result<(), &'static str> {
            // Process the request
            Ok(())
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq, Eq)]
    pub struct FSEvent {
        pub creationtime: [u8; 64],
        pub filepath: [u8; 256],
        pub eventkey: [u8; 128],
    }

    #[pallet::storage]
    #[pallet::getter(fn info)]
    pub type DisReAssembly<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, FSEvent, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {
        CreationTimeTooLong,
        FilePathTooLong,
        EventKeyTooLong,
    }

	/*#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			// Your offchain worker logic here
			//log::info!("Offchain Worker running at block number: {:?}", block_number);

			// Example: Process disassembly requests
			let requests = Self::fetch_disassembly_requests();
			for request in requests {
				if let Err(e) = Self::process_disassembly_request(request) {
					//log::error!("Failed to process disassembly request: {:?}", e);
				}
			}
		}
	}*/

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn disassembled(
            origin: OriginFor<T>,
            creation_time: Vec<u8>,
            file_path: Vec<u8>,
            event_key: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(creation_time.len() <= 64, Error::<T>::CreationTimeTooLong);
            ensure!(file_path.len() <= 256, Error::<T>::FilePathTooLong);
            ensure!(event_key.len() <= 128, Error::<T>::EventKeyTooLong);

            let event = FSEvent {
                creationtime: {
                    let mut arr = [0u8; 64];
                    arr[..creation_time.len()].copy_from_slice(&creation_time);
                    arr
                },
                filepath: {
                    let mut arr = [0u8; 256];
                    arr[..file_path.len()].copy_from_slice(&file_path);
                    arr
                },
                eventkey: {
                    let mut arr = [0u8; 128];
                    arr[..event_key.len()].copy_from_slice(&event_key);
                    arr
                },
            };

            <DisReAssembly<T>>::insert(&sender, &event);

            Self::deposit_event(Event::<T>::FileDisassembled { who: sender.clone(), event: event.clone() });
		
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn reassembled(
            origin: OriginFor<T>,
            creation_time: Vec<u8>,
            file_path: Vec<u8>,
            event_key: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(creation_time.len() <= 64, Error::<T>::CreationTimeTooLong);
            ensure!(file_path.len() <= 256, Error::<T>::FilePathTooLong);
            ensure!(event_key.len() <= 128, Error::<T>::EventKeyTooLong);

            let event = FSEvent {
                creationtime: {
                    let mut arr = [0u8; 64];
                    arr[..creation_time.len()].copy_from_slice(&creation_time);
                    arr
                },
                filepath: {
                    let mut arr = [0u8; 256];
                    arr[..file_path.len()].copy_from_slice(&file_path);
                    arr
                },
                eventkey: {
                    let mut arr = [0u8; 128];
                    arr[..event_key.len()].copy_from_slice(&event_key);
                    arr
                },
            };

            <DisReAssembly<T>>::insert(&sender, &event);

            Self::deposit_event(Event::<T>::FileReassembled { who: sender.clone(), event: event.clone() });

            Ok(())
        }

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn request_disassembly(
			origin: OriginFor<T>,
			creation_time: Vec<u8>,
			file_path: Vec<u8>,
			event_key: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
	
			// Ensure the data sizes are within limits
			ensure!(creation_time.len() <= 64, Error::<T>::CreationTimeTooLong);
			ensure!(file_path.len() <= 256, Error::<T>::FilePathTooLong);
			ensure!(event_key.len() <= 128, Error::<T>::EventKeyTooLong);
	
			// Construct the FSEvent
			let event = FSEvent {
				creationtime: {
					let mut arr = [0u8; 64];
					arr[..creation_time.len()].copy_from_slice(&creation_time);
					arr
				},
				filepath: {
					let mut arr = [0u8; 256];
					arr[..file_path.len()].copy_from_slice(&file_path);
					arr
				},
				eventkey: {
					let mut arr = [0u8; 128];
					arr[..event_key.len()].copy_from_slice(&event_key);
					arr
				},
			};
	
			// Emit an event to log the request
			Self::deposit_event(Event::DisassemblyRequested { who, event });
	
			Ok(())
		}
	}
}