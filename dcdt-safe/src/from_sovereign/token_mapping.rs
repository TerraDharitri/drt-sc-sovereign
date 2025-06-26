dharitri_sc::imports!();
dharitri_sc::derive_imports!();

const DEFAULT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 REWA
const REGISTER_GAS: u64 = 60_000_000;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct DcdtInfo<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
}

struct IssueDcdtArgs<M: ManagedTypeApi> {
    sov_token_id: TokenIdentifier<M>,
    token_type: DcdtTokenType,
    issue_cost: BigUint<M>,
    token_display_name: ManagedBuffer<M>,
    token_ticker: ManagedBuffer<M>,
    num_decimals: usize,
}

#[dharitri_sc::module]
pub trait TokenMappingModule: utils::UtilsModule {
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
        let is_sovereign_chain = self.is_sovereign_chain().get();
        require!(
            !is_sovereign_chain,
            "Invalid method to call in current chain"
        );

        self.require_token_has_prefix(&sov_token_id);

        let issue_cost = self.call_value().rewa_value().clone_value();
        require!(
            issue_cost == DEFAULT_ISSUE_COST,
            "rEWA value should be 0.05"
        );

        self.require_sov_token_id_not_registered(&sov_token_id);

        match token_type {
            DcdtTokenType::Invalid => sc_panic!("Invalid type"),
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
            .callback(
                <Self as TokenMappingModule>::callbacks(self).issue_callback(&args.sov_token_id),
            )
            .register_promise();
    }

    #[promises_callback]
    fn issue_callback(
        &self,
        sov_token_id: &TokenIdentifier,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier<Self::Api>>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(mvx_token_id) => {
                self.set_corresponding_token_ids(sov_token_id, &mvx_token_id);
            }
            ManagedAsyncCallResult::Err(error) => {
                sc_panic!("There was an error at issuing token: '{}'", error.err_msg);
            }
        }
    }

    fn set_corresponding_token_ids(
        &self,
        sov_token_id: &TokenIdentifier,
        mvx_token_id: &TokenIdentifier,
    ) {
        self.sovereign_to_dharitri_token_id_mapper(sov_token_id)
            .set(mvx_token_id);

        self.dharitri_to_sovereign_token_id_mapper(mvx_token_id)
            .set(sov_token_id);
    }

    fn update_dcdt_info_mappers(
        &self,
        sov_id: &TokenIdentifier,
        sov_nonce: u64,
        mvx_id: &TokenIdentifier,
        new_nft_nonce: u64,
    ) {
        self.sovereign_to_dharitri_dcdt_info_mapper(sov_id, sov_nonce)
            .set(DcdtInfo {
                token_identifier: mvx_id.clone(),
                token_nonce: new_nft_nonce,
            });

        self.dharitri_to_sovereign_dcdt_info_mapper(mvx_id, new_nft_nonce)
            .set(DcdtInfo {
                token_identifier: sov_id.clone(),
                token_nonce: sov_nonce,
            });
    }

    #[inline]
    fn clear_sov_to_mvx_dcdt_info_mapper(&self, id: &TokenIdentifier, nonce: u64) {
        self.sovereign_to_dharitri_dcdt_info_mapper(id, nonce)
            .take();
    }

    #[inline]
    fn clear_mvx_to_sov_dcdt_info_mapper(&self, id: &TokenIdentifier, nonce: u64) {
        self.dharitri_to_sovereign_dcdt_info_mapper(id, nonce)
            .take();
    }

    #[inline]
    fn is_fungible(self, token_type: &DcdtTokenType) -> bool {
        *token_type == DcdtTokenType::Fungible
    }

    #[inline]
    fn is_sft_or_meta(self, token_type: &DcdtTokenType) -> bool {
        *token_type == DcdtTokenType::SemiFungible || *token_type == DcdtTokenType::Meta
    }

    #[inline]
    fn is_nft(self, token_type: &DcdtTokenType) -> bool {
        *token_type == DcdtTokenType::NonFungible
    }

    #[inline]
    fn require_sov_token_id_not_registered(&self, id: &TokenIdentifier) {
        require!(
            self.sovereign_to_dharitri_token_id_mapper(id).is_empty(),
            "This token was already registered"
        );
    }

    #[storage_mapper("sovToDrtTokenId")]
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

    #[storage_mapper("isSovereignChain")]
    fn is_sovereign_chain(&self) -> SingleValueMapper<bool>;
}
