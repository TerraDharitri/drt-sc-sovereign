#![no_std]

use dharitri_sc::imports::*;
use structs::configs::DcdtSafeConfig;

pub mod bridging_mechanism;
pub mod deposit;
pub mod execute;
pub mod register_token;

#[dharitri_sc::contract]
pub trait DrtDcdtSafe:
    deposit::DepositModule
    + cross_chain::LibCommon
    + execute::ExecuteModule
    + register_token::RegisterTokenModule
    + bridging_mechanism::BridgingMechanism
    + cross_chain::deposit_common::DepositCommonModule
    + cross_chain::events::EventsModule
    + cross_chain::storage::CrossChainStorage
    + cross_chain::execute_common::ExecuteCommonModule
    + dharitri_sc_modules::pause::PauseModule
    + utils::UtilsModule
    + dharitri_sc_modules::only_admin::OnlyAdminModule
{
    #[init]
    fn init(
        &self,
        header_verifier_address: ManagedAddress,
        opt_config: OptionalValue<DcdtSafeConfig<Self::Api>>,
    ) {
        self.require_sc_address(&header_verifier_address);
        self.header_verifier_address().set(&header_verifier_address);
        self.admins().insert(self.blockchain().get_caller());

        self.dcdt_safe_config().set(
            opt_config
                .into_option()
                .inspect(|config| self.require_dcdt_config_valid(config))
                .unwrap_or_else(DcdtSafeConfig::default_config),
        );

        self.set_paused(true);
    }

    #[only_admin]
    #[endpoint(updateConfiguration)]
    fn update_configuration(&self, new_config: DcdtSafeConfig<Self::Api>) {
        self.require_dcdt_config_valid(&new_config);
        self.dcdt_safe_config().set(new_config);
    }

    #[only_admin]
    #[endpoint(setFeeMarketAddress)]
    fn set_fee_market_address(&self, fee_market_address: ManagedAddress) {
        self.require_sc_address(&fee_market_address);
        self.fee_market_address().set(fee_market_address);
    }

    #[only_admin]
    #[endpoint(setMaxBridgedAmount)]
    fn set_max_bridged_amount(&self, token_id: TokenIdentifier, max_amount: BigUint) {
        self.max_bridged_amount(&token_id).set(&max_amount);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
