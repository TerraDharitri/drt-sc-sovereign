use proxies::fee_market_proxy::FeeType;
use structs::{aliases::TxNonce, configs::DcdtSafeConfig, DcdtInfo};

dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait CrossChainStorage {
    #[storage_mapper("lastTxNonce")]
    fn last_tx_nonce(&self) -> SingleValueMapper<TxNonce>;

    #[storage_mapper("crossChainConfig")]
    fn dcdt_safe_config(&self) -> SingleValueMapper<DcdtSafeConfig<Self::Api>>;

    #[storage_mapper("feeMarketAddress")]
    fn fee_market_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("headerVerifierAddress")]
    fn header_verifier_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("sovToMxTokenId")]
    fn sovereign_to_dharitri_token_id_mapper(
        &self,
        sov_token_id: &TokenIdentifier,
    ) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("drtToSovTokenId")]
    fn dharitri_to_sovereign_token_id_mapper(
        &self,
        drt_token_id: &TokenIdentifier,
    ) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("sovDcdtTokenInfoMapper")]
    fn sovereign_to_dharitri_dcdt_info_mapper(
        &self,
        token_identifier: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<DcdtInfo<Self::Api>>;

    #[storage_mapper("drtDcdtTokenInfoMapper")]
    fn dharitri_to_sovereign_dcdt_info_mapper(
        &self,
        token_identifier: &TokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<DcdtInfo<Self::Api>>;

    #[view(getNativeToken)]
    #[storage_mapper("nativeToken")]
    fn native_token(&self) -> SingleValueMapper<TokenIdentifier<Self::Api>>;

    #[storage_mapper("isSovereignChain")]
    fn is_sovereign_chain(&self) -> SingleValueMapper<bool>;

    #[view(getMaxBridgedAmount)]
    #[storage_mapper("maxBridgedAmount")]
    fn max_bridged_amount(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper_from_address("feeEnabledFlag")]
    fn external_fee_enabled(
        &self,
        sc_address: ManagedAddress,
    ) -> SingleValueMapper<bool, ManagedAddress>;

    #[storage_mapper_from_address("tokenFee")]
    fn external_token_fee(
        &self,
        sc_address: ManagedAddress,
        token_id: &TokenIdentifier,
    ) -> SingleValueMapper<FeeType<Self::Api>, ManagedAddress>;
}
