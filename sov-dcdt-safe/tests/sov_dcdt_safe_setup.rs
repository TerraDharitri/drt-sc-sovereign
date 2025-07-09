use dharitri_sc::{
    imports::OptionalValue,
    types::{
        DcdtLocalRole, ManagedAddress, ManagedVec, TestSCAddress, TestTokenIdentifier,
        TokenIdentifier,
    },
};

use dharitri_sc_scenario::{
    api::StaticApi, ReturnsHandledOrError, ReturnsLogs, ScenarioTxRun, ScenarioTxWhitebox,
};

use common_test_setup::constants::{
    DCDT_SAFE_ADDRESS, FEE_MARKET_ADDRESS, FEE_TOKEN, FIRST_TEST_TOKEN, ONE_HUNDRED_MILLION,
    OWNER_ADDRESS, OWNER_BALANCE, SECOND_TEST_TOKEN, SOV_DCDT_SAFE_CODE_PATH, USER,
};
use common_test_setup::{AccountSetup, BaseSetup};
use proxies::sov_dcdt_safe_proxy::SovDcdtSafeProxy;
use sov_dcdt_safe::SovDcdtSafe;
use structs::{
    aliases::{OptionalValueTransferDataTuple, PaymentsVec},
    configs::DcdtSafeConfig,
};

pub struct SovDcdtSafeTestState {
    pub common_setup: BaseSetup,
}

impl SovDcdtSafeTestState {
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
            .register_contract(SOV_DCDT_SAFE_CODE_PATH, sov_dcdt_safe::ContractBuilder);

        Self { common_setup }
    }

    pub fn deploy_contract(
        &mut self,
        fee_market_address: TestSCAddress,
        opt_config: OptionalValue<DcdtSafeConfig<StaticApi>>,
    ) -> &mut Self {
        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(SovDcdtSafeProxy)
            .init(fee_market_address, opt_config)
            .code(SOV_DCDT_SAFE_CODE_PATH)
            .new_address(DCDT_SAFE_ADDRESS)
            .run();

        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(SovDcdtSafeProxy)
            .unpause_endpoint()
            .run();

        self
    }

    pub fn deploy_contract_with_roles(&mut self) -> &mut Self {
        self.common_setup
            .world
            .account(DCDT_SAFE_ADDRESS)
            .nonce(1)
            .code(SOV_DCDT_SAFE_CODE_PATH)
            .owner(OWNER_ADDRESS)
            .dcdt_roles(
                TokenIdentifier::from(FIRST_TEST_TOKEN),
                vec![
                    DcdtLocalRole::Burn.name().to_string(),
                    DcdtLocalRole::NftBurn.name().to_string(),
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
            .whitebox(sov_dcdt_safe::contract_obj, |sc| {
                let config = DcdtSafeConfig::new(
                    ManagedVec::new(),
                    ManagedVec::new(),
                    50_000_000,
                    ManagedVec::new(),
                );

                sc.init(
                    FEE_MARKET_ADDRESS.to_managed_address(),
                    OptionalValue::Some(config),
                );
            });

        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(SovDcdtSafeProxy)
            .unpause_endpoint()
            .run();

        self
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
            .typed(SovDcdtSafeProxy)
            .deposit(to, opt_transfer_data.clone())
            .payment(payment)
            .returns(ReturnsLogs)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, expected_error_message);

        if let Some(custom_log) = expected_custom_log {
            self.common_setup.assert_expected_log(logs, custom_log)
        };
    }

    pub fn set_fee_market_address(&mut self, fee_market_address: TestSCAddress) {
        self.common_setup
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .to(DCDT_SAFE_ADDRESS)
            .typed(SovDcdtSafeProxy)
            .set_fee_market_address(fee_market_address)
            .run();
    }

    pub fn deposit_with_logs(
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
            .typed(SovDcdtSafeProxy)
            .deposit(to, opt_transfer_data)
            .payment(payment)
            .returns(ReturnsLogs)
            .returns(ReturnsHandledOrError::new())
            .run();

        self.common_setup
            .assert_expected_error_message(response, expected_error_message);

        if let Some(custom_log) = expected_custom_log {
            self.common_setup.assert_expected_log(logs, custom_log)
        };
    }
}
