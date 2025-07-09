use error_messages::NO_HEADER_VERIFIER_ADDRESS;
use proxies::header_verifier_proxy::HeaderverifierProxy;
use structs::operation::Operation;

dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait ExecuteCommonModule: crate::storage::CrossChainStorage {
    fn calculate_operation_hash(&self, operation: &Operation<Self::Api>) -> ManagedBuffer {
        let mut serialized_data = ManagedBuffer::new();

        if let core::result::Result::Err(err) = operation.top_encode(&mut serialized_data) {
            sc_panic!("Transfer data encode error: {}", err.message_bytes());
        }

        let sha256 = self.crypto().sha256(&serialized_data);
        let hash = sha256.as_managed_buffer().clone();

        hash
    }

    fn lock_operation_hash(&self, operation_hash: &ManagedBuffer, hash_of_hashes: &ManagedBuffer) {
        self.tx()
            .to(self.get_header_verifier_address())
            .typed(HeaderverifierProxy)
            .lock_operation_hash(hash_of_hashes, operation_hash)
            .sync_call();
    }

    fn remove_executed_hash(&self, hash_of_hashes: &ManagedBuffer, op_hash: &ManagedBuffer) {
        self.tx()
            .to(self.get_header_verifier_address())
            .typed(HeaderverifierProxy)
            .remove_executed_hash(hash_of_hashes, op_hash)
            .sync_call();
    }

    fn get_header_verifier_address(&self) -> ManagedAddress {
        let header_verifier_address_mapper = self.header_verifier_address();

        require!(
            !header_verifier_address_mapper.is_empty(),
            NO_HEADER_VERIFIER_ADDRESS
        );

        header_verifier_address_mapper.get()
    }

    fn is_native_token(&self, token_identifier: &TokenIdentifier) -> bool {
        let dcdt_safe_native_token_mapper = self.native_token();

        if dcdt_safe_native_token_mapper.is_empty() {
            return false;
        }

        token_identifier == &dcdt_safe_native_token_mapper.get()
    }

    #[inline]
    fn is_fungible(self, token_type: &DcdtTokenType) -> bool {
        *token_type == DcdtTokenType::Fungible
    }

    #[inline]
    fn is_sft_or_meta(self, token_type: &DcdtTokenType) -> bool {
        *token_type == DcdtTokenType::SemiFungible
            || *token_type == DcdtTokenType::DynamicSFT
            || *token_type == DcdtTokenType::Meta
            || *token_type == DcdtTokenType::DynamicMeta
    }

    #[inline]
    fn is_nft(self, token_type: &DcdtTokenType) -> bool {
        *token_type == DcdtTokenType::NonFungible || *token_type == DcdtTokenType::DynamicNFT
    }
}
