#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// --- substrate ---
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, traits::Get,
	weights::Weight,
};

pub trait Trait<I: Instance = DefaultInstance>: frame_system::Trait {
	type Event: From<Event<Self, I>> + Into<<Self as frame_system::Trait>::Event>;
	type TemplateConst: Get<u32>;
	type WeightInfo: WeightInfo;
}

pub trait WeightInfo {
	fn template_call() -> Weight;
}
impl WeightInfo for () {
	fn template_call() -> Weight {
		0
	}
}

decl_error! {
	pub enum Error for Module<T: Trait<I>, I: Instance> {
		/// Template error doc - REASON
		TemplateError,
	}
}

decl_event! {
	pub enum Event<T, I: Instance = DefaultInstance>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
	{
		/// Template event doc. [user]
		TemplateEvent(AccountId),
	}
}

decl_storage! {
	trait Store for Module<T: Trait<I>, I: Instance = DefaultInstance> as TemplateModule {
		pub TemplateStorageValue get(fn template_storage_value): u32;

		pub TemplateStorageMap
			get(fn template_storage_map)
			: map hasher(identity) u32
			=> u32;

		pub TemplateStorageDoubleMap
			get(fn template_storage_double_map)
			: double_map
				hasher(identity) u32,
				hasher(identity) u32
			=> u32;
	}
}

decl_module! {
	pub struct Module<T: Trait<I>, I: Instance = DefaultInstance> for enum Call
	where
		origin: T::Origin
	{
		type Error = Error<T, I>;

		const TEMPLATE_CONST:u32 = T::TemplateConst::get();

		fn deposit_event() = default;

		#[weight = 0]
		pub fn template_call(origin, result: DispatchResult)  {
			result?;
		}
	}
}
