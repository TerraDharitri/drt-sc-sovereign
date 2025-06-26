#![no_std]

dharitri_sc::imports!();

pub type PaymentsVec<M> = ManagedVec<M, DcdtTokenPayment<M>>;

static ERR_EMPTY_PAYMENTS: &[u8] = b"No payments";
const DASH: u8 = b'-';
const MAX_TOKEN_ID_LEN: usize = 32;

#[dharitri_sc::module]
pub trait UtilsModule {
    fn require_sc_address(&self, address: &ManagedAddress) {
        require!(
            !address.is_zero() && self.blockchain().is_smart_contract(address),
            "Invalid SC address"
        );
    }

    fn require_valid_token_id(&self, token_id: &TokenIdentifier) {
        require!(token_id.is_valid_dcdt_identifier(), "Invalid token ID");
    }

    fn remove_items<
        T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
        I: IntoIterator<Item = T>,
    >(
        &self,
        mapper: &mut UnorderedSetMapper<T>,
        items: I,
    ) {
        for item in items {
            let was_removed = mapper.swap_remove(&item);
            require!(was_removed, "Item not found in list");
        }
    }

    fn pop_first_payment(
        &self,
        payments: &mut PaymentsVec<Self::Api>,
    ) -> DcdtTokenPayment<Self::Api> {
        require!(!payments.is_empty(), ERR_EMPTY_PAYMENTS);

        let first_payment = payments.get(0);
        payments.remove(0);

        first_payment
    }

    fn has_prefix(&self, token_id: &TokenIdentifier) -> bool {
        let buffer = token_id.as_managed_buffer();
        let mut array_buffer = [0u8; MAX_TOKEN_ID_LEN];
        let slice = buffer.load_to_byte_array(&mut array_buffer);

        let counter = slice.iter().filter(|&&c| c == DASH).count();

        if counter == 2 {
            return true;
        }

        false
    }

    #[inline]
    fn require_token_has_prefix(&self, token_id: &TokenIdentifier) {
        require!(self.has_prefix(token_id), "Token Id does not have prefix");
    }

    fn has_sov_prefix(&self, token_id: &TokenIdentifier, chain_prefix: &ManagedBuffer) -> bool {
        if !self.has_prefix(token_id) {
            return false;
        }

        let buffer = token_id.as_managed_buffer();
        let mut array_buffer = [0u8; MAX_TOKEN_ID_LEN];
        let slice = buffer.load_to_byte_array(&mut array_buffer);

        if let Some(index) = slice.iter().position(|&b| b == DASH) {
            let prefix = ManagedBuffer::from(&slice[..index]);

            if prefix == chain_prefix.clone() {
                return true;
            }
        }

        false
    }
}
