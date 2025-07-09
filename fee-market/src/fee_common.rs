use error_messages::ONLY_DCDT_SAFE_CALLER;

dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait CommonFeeModule {
    fn require_caller_dcdt_safe(&self) {
        let caller = self.blockchain().get_caller();
        let dcdt_safe_address = self.dcdt_safe_address().get();
        require!(caller == dcdt_safe_address, ONLY_DCDT_SAFE_CALLER);
    }

    #[storage_mapper("dcdtSafeAddress")]
    fn dcdt_safe_address(&self) -> SingleValueMapper<ManagedAddress>;
}
