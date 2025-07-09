use cross_chain::REGISTER_GAS;
use error_messages::{CANNOT_REGISTER_TOKEN, INVALID_TYPE, NATIVE_TOKEN_ALREADY_REGISTERED};
use dharitri_sc::types::DcdtTokenType;
use structs::{DcdtInfo, IssueDcdtArgs};
dharitri_sc::imports!();
dharitri_sc::derive_imports!();

#[dharitri_sc::module]
pub trait RegisterTokenModule:
    utils::UtilsModule
    + cross_chain::storage::CrossChainStorage
    + cross_chain::deposit_common::DepositCommonModule
    + cross_chain::execute_common::ExecuteCommonModule
{
    #[payable("REWA")]
    #[endpoint(registerToken)]
    fn register_token(
        &self,
        sov_token_id: TokenIdentifier,
        token_type: DcdtTokenType,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        self.require_sov_token_id_not_registered(&sov_token_id);

        require!(self.has_prefix(&sov_token_id), CANNOT_REGISTER_TOKEN);
        let issue_cost = self.call_value().rewa().clone_value();

        match token_type {
            DcdtTokenType::Invalid => sc_panic!(INVALID_TYPE),
            _ => self.handle_token_issue(IssueDcdtArgs {
                sov_token_id: sov_token_id.clone(),
                issue_cost,
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            }),
        }
    }

    #[payable("REWA")]
    #[only_owner]
    #[endpoint(registerNativeToken)]
    fn register_native_token(&self, token_ticker: ManagedBuffer, token_name: ManagedBuffer) {
        require!(
            self.native_token().is_empty(),
            NATIVE_TOKEN_ALREADY_REGISTERED
        );

        self.tx()
            .to(DCDTSystemSCAddress)
            .typed(DCDTSystemSCProxy)
            .issue_and_set_all_roles(
                self.call_value().rewa().clone_value(),
                token_name,
                &token_ticker,
                DcdtTokenType::Fungible,
                18,
            )
            .gas(REGISTER_GAS)
            .callback(self.callbacks().native_token_issue_callback())
            .register_promise();
    }

    fn handle_token_issue(&self, args: IssueDcdtArgs<Self::Api>) {
        self.tx()
            .to(DCDTSystemSCAddress)
            .typed(DCDTSystemSCProxy)
            .issue_and_set_all_roles(
                args.issue_cost,
                args.token_display_name,
                args.token_ticker,
                args.token_type,
                args.num_decimals,
            )
            .gas(REGISTER_GAS)
            .callback(self.callbacks().issue_callback(&args.sov_token_id))
            .register_promise();
    }

    #[promises_callback]
    fn issue_callback(
        &self,
        sov_token_id: &TokenIdentifier,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier<Self::Api>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(drt_token_id) => {
                self.set_corresponding_token_ids(sov_token_id, &drt_token_id);
            }
            ManagedAsyncCallResult::Err(error) => {
                sc_panic!("There was an error at issuing token: '{}'", error.err_msg);
            }
        }
    }

    #[promises_callback]
    fn native_token_issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier<Self::Api>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(native_token_id) => {
                self.native_token().set(native_token_id);
            }
            ManagedAsyncCallResult::Err(error) => {
                sc_panic!(
                    "There was an error at issuing native token: '{}'",
                    error.err_msg
                );
            }
        }
    }
    fn set_corresponding_token_ids(
        &self,
        sov_token_id: &TokenIdentifier,
        drt_token_id: &TokenIdentifier,
    ) {
        self.sovereign_to_dharitri_token_id_mapper(sov_token_id)
            .set(drt_token_id);

        self.dharitri_to_sovereign_token_id_mapper(drt_token_id)
            .set(sov_token_id);
    }

    fn update_dcdt_info_mappers(
        &self,
        sov_id: &TokenIdentifier,
        sov_nonce: u64,
        drt_id: &TokenIdentifier,
        new_nft_nonce: u64,
    ) {
        self.sovereign_to_dharitri_dcdt_info_mapper(sov_id, sov_nonce)
            .set(DcdtInfo {
                token_identifier: drt_id.clone(),
                token_nonce: new_nft_nonce,
            });

        self.dharitri_to_sovereign_dcdt_info_mapper(drt_id, new_nft_nonce)
            .set(DcdtInfo {
                token_identifier: sov_id.clone(),
                token_nonce: sov_nonce,
            });
    }
}
