#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{Interface, ProvideInherent};
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::interface]
	impl<T: Config> Interface<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {}
}

fn main() {
}
