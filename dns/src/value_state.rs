use dharitri_sc::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::ManagedAddress,
};

dharitri_sc::derive_imports!();

#[derive(
    NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Debug, Clone,
)]
pub enum ValueState<M: ManagedTypeApi> {
    None,
    PendingDharitri(ManagedAddress<M>),
    RegisteredDharitri(ManagedAddress<M>),
    PendingMigration(ManagedAddress<M>),
    PendingX(ManagedAddress<M>),
    RegisteredX(ManagedAddress<M>),
}

impl<M: ManagedTypeApi> ValueState<M> {
    pub fn is_available(&self) -> bool {
        matches!(self, ValueState::None)
    }

    pub fn start_migration(&mut self) -> ManagedAddress<M> {
        let result;
        *self = if let ValueState::RegisteredDharitri(address) = self {
            result = address.clone();
            ValueState::PendingMigration(address.clone())
        } else {
            M::error_api_impl().signal_error(b"can't migrate")
        };
        result
    }

    pub fn finalize(&mut self) {
        *self = match self {
            Self::PendingDharitri(address) => Self::RegisteredDharitri(address.clone()),
            Self::PendingMigration(address) => Self::RegisteredX(address.clone()),
            Self::PendingX(address) => Self::RegisteredX(address.clone()),
            _ => Self::None,
        };
    }

    pub fn revert(&mut self) {
        *self = match self {
            Self::PendingDharitri(_) => Self::None,
            Self::PendingMigration(address) => Self::RegisteredDharitri(address.clone()),
            Self::PendingX(_) => Self::None,
            _ => Self::None,
        }
    }
}
