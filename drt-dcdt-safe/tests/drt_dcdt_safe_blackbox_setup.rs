use common_test_setup::constants::{
    DCDT_SAFE_ADDRESS, FEE_TOKEN, FIRST_TEST_TOKEN, HEADER_VERIFIER_ADDRESS,
    DRT_DCDT_SAFE_CODE_PATH, ONE_HUNDRED_MILLION, OWNER_ADDRESS, OWNER_BALANCE, SECOND_TEST_TOKEN,
    USER,
};
use common_test_setup::{AccountSetup, BaseSetup, RegisterTokenArgs};
use dharitri_sc::{
    codec::TopEncode,
    imports::OptionalValue,
    types::{
        BigUint, DcdtLocalRole, ManagedAddress, ManagedBuffer, ManagedVec, MultiValueEncoded,
        TestSCAddress, TestTokenIdentifier, TokenIdentifier,
    },
};
use dharitri_sc_modules::transfer_role_proxy::PaymentsVec;
use dharitri_sc_scenario::{
    api::StaticApi, dharitri_chain_vm::crypto_functions::sha256, ReturnsHandledOrError,
    ReturnsLogs, ScenarioTxRun, ScenarioTxWhitebox,
};
use drt_dcdt_safe::{bridging_mechanism::TRUSTED_TOKEN_IDS, DrtDcdtSafe};
use proxies::{header_verifier_proxy::HeaderverifierProxy, drt_dcdt_safe_proxy::DrtDcdtSafeProxy};
use structs::{
    aliases::OptionalValueTransferDataTuple, configs::DcdtSafeConfig, operation::Operation,
};

pub struct DrtDcdtSafeTestState {
    pub common_setup: BaseSetup,
}

impl DrtDcdtSafeTestState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let owner_account = AccountSetup {
            address: OWNER_ADDRESS,
            dcdt_balances: Some(vec![
                (
                    TestTokenIdentifier::new(FIRST_TEST_TOKEN),
                    ONE_HUNDRED_MILLION.into(),
                ),
                (
                    TestTokenIdentifier::new(SECOND_TEST_TOKEN),
                    ONE_HUNDRED_MILLION.into(),
                ),
                (
                    TestTokenIdentifier::new(FEE_TOKEN),
                    ONE_HUNDRED_MILLION.into(),
                ),
                (
                    TestTokenIdentifier::new(TRUSTED_TOKEN_IDS[0]),
                    ONE_HUNDRED_MILLION.into(),
                ),
            ]),
            rewa_balance: Some(OWNER_BALANCE.into()),
        };

        let user_account = AccountSetup {
            address: USER,
            dcdt_balances: Some(vec![(
                TestTokenIdentifier::new(FIRST_TEST_TOKEN),
                ONE_HUNDRED_MILLION.into(),
            )]),
            rewa_balance: Some(OWNER_BALANCE.into()),
        };

        let account_setups = vec![owner_account, user_account];

        let mut common_setup = BaseSetup::new(account_setups);

        common_setup
            .world
            .register_contract(DRT_DCDT_SAFE_CODE_PATH, drt_dcdt_safe::ContractBuilder);

        Self { common_setup }
    }

    pub fn deploy_contract(
        &mut self,
        header_verifier_address: TestSCAddress,
        opt_config: OptionalValue<DcdtSafeConfig<StaticApi>>,
    ) -> &mut Self {
        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .init(header_verifier_address, opt_config)
            .code(DRT_DCDT_SAFE_CODE_PATH)
            .new_address(DCDT_SAFE_ADDRESS)
            .run();

        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .unpause_endpoint()
            .run();

        self
    }

    pub fn deploy_contract_with_roles(&mut self) -> &mut Self {
        self.common_setup
            .world
            .account(DCDT_SAFE_ADDRESS)
            .nonce(1)
            .code(DRT_DCDT_SAFE_CODE_PATH)
            .owner(OWNER_ADDRESS)
            .dcdt_roles(
                TokenIdentifier::from(FIRST_TEST_TOKEN),
                vec![
                    DcdtLocalRole::Burn.name().to_string(),
                    DcdtLocalRole::NftBurn.name().to_string(),
                    DcdtLocalRole::Mint.name().to_string(),
                ],
            )
            .dcdt_roles(
                TokenIdentifier::from(TRUSTED_TOKEN_IDS[0]),
                vec![
                    DcdtLocalRole::Burn.name().to_string(),
                    DcdtLocalRole::NftBurn.name().to_string(),
                    DcdtLocalRole::Mint.name().to_string(),
                ],
            )
            .dcdt_roles(
                TokenIdentifier::from(SECOND_TEST_TOKEN),
                vec![
                    DcdtLocalRole::Burn.name().to_string(),
                    DcdtLocalRole::NftBurn.name().to_string(),
                ],
            )
            .dcdt_roles(
                TokenIdentifier::from(FEE_TOKEN),
                vec![
                    DcdtLocalRole::Burn.name().to_string(),
                    DcdtLocalRole::NftBurn.name().to_string(),
                ],
            );

        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .whitebox(drt_dcdt_safe::contract_obj, |sc| {
                let config = DcdtSafeConfig::new(
                    ManagedVec::new(),
                    ManagedVec::new(),
                    50_000_000,
                    ManagedVec::new(),
                );

                sc.init(
                    HEADER_VERIFIER_ADDRESS.to_managed_address(),
                    OptionalValue::Some(config),
                );
            });

        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .unpause_endpoint()
            .run();

        self
    }

    pub fn update_configuration(
        &mut self,
        new_config: DcdtSafeConfig<StaticApi>,
        err_message: Option<&str>,
    ) {
        let response = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .update_configuration(new_config)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, err_message);
    }

    pub fn set_token_burn_mechanism(
        &mut self,
        token_id: &str,
        expected_error_message: Option<&str>,
    ) -> &mut Self {
        let response = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .set_token_burn_mechanism(TokenIdentifier::from(token_id))
            .returns(ReturnsHandledOrError::new())
            .run();

        match response {
            Ok(_) => assert!(
                expected_error_message.is_none(),
                "Transaction was successful, but expected error"
            ),
            Err(error) => {
                assert_eq!(expected_error_message, Some(error.message.as_str()))
            }
        }

        self
    }

    pub fn set_token_lock_mechanism(
        &mut self,
        token_id: &str,
        expected_error_message: Option<&str>,
    ) -> &mut Self {
        let response = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .set_token_lock_mechanism(TokenIdentifier::from(token_id))
            .returns(ReturnsHandledOrError::new())
            .run();

        match response {
            Ok(_) => assert!(
                expected_error_message.is_none(),
                "Transaction was successful, but expected error"
            ),
            Err(error) => {
                assert_eq!(expected_error_message, Some(error.message.as_str()))
            }
        }

        self
    }

    pub fn set_fee_market_address(&mut self, fee_market_address: TestSCAddress) {
        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .set_fee_market_address(fee_market_address)
            .run();
    }

    pub fn deposit(
        &mut self,
        to: ManagedAddress<StaticApi>,
        opt_transfer_data: OptionalValueTransferDataTuple<StaticApi>,
        payment: PaymentsVec<StaticApi>,
        expected_error_message: Option<&str>,
        expected_custom_log: Option<&str>,
    ) {
        let (logs, response) = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .deposit(to, opt_transfer_data.clone())
            .payment(payment.clone())
            .returns(ReturnsLogs)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, expected_error_message);

        if let Some(custom_log) = expected_custom_log {
            self.common_setup.assert_expected_log(logs, custom_log)
        };
    }

    pub fn register_token(
        &mut self,
        register_token_args: RegisterTokenArgs,
        payment: BigUint<StaticApi>,
        expected_error_message: Option<&str>,
    ) {
        let response = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .register_token(
                register_token_args.sov_token_id,
                register_token_args.token_type,
                ManagedBuffer::from(register_token_args.token_display_name),
                ManagedBuffer::from(register_token_args.token_ticker),
                register_token_args.num_decimals,
            )
            .rewa(payment)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, expected_error_message);
    }

    pub fn register_native_token(
        &mut self,
        token_ticker: &str,
        token_name: &str,
        payment: BigUint<StaticApi>,
        expected_error_message: Option<&str>,
    ) {
        let response = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .register_native_token(
                ManagedBuffer::from(token_ticker),
                ManagedBuffer::from(token_name),
            )
            .rewa(payment)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, expected_error_message);
    }

    pub fn execute_operation(
        &mut self,
        hash_of_hashes: &ManagedBuffer<StaticApi>,
        operation: &Operation<StaticApi>,
        expected_error_message: Option<&str>,
        expected_custom_log: Option<&str>,
    ) {
        let (logs, response) = self
            .common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(DrtDcdtSafeProxy)
            .execute_operations(hash_of_hashes, operation)
            .returns(ReturnsLogs)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, expected_error_message);

        if let Some(custom_log) = expected_custom_log {
            self.common_setup.assert_expected_log(logs, custom_log)
        };
    }

    pub fn set_dcdt_safe_address_in_header_verifier(&mut self, dcdt_safe_address: TestSCAddress) {
        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(HEADER_VERIFIER_ADDRESS)
            .typed(HeaderverifierProxy)
            .set_dcdt_safe_address(dcdt_safe_address)
            .run();
    }

    pub fn register_operation(
        &mut self,
        signature: ManagedBuffer<StaticApi>,
        hash_of_hashes: &ManagedBuffer<StaticApi>,
        operations_hashes: MultiValueEncoded<StaticApi, ManagedBuffer<StaticApi>>,
    ) {
        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(HEADER_VERIFIER_ADDRESS)
            .typed(HeaderverifierProxy)
            .register_bridge_operations(
                signature,
                hash_of_hashes,
                ManagedBuffer::new(),
                ManagedBuffer::new(),
                operations_hashes,
            )
            .run();
    }

    pub fn get_operation_hash(
        &mut self,
        operation: &Operation<StaticApi>,
    ) -> ManagedBuffer<StaticApi> {
        let mut serialized_operation: ManagedBuffer<StaticApi> = ManagedBuffer::new();
        let _ = operation.top_encode(&mut serialized_operation);
        let sha256 = sha256(&serialized_operation.to_vec());

        ManagedBuffer::new_from_bytes(&sha256)
    }
}
