
#[test_only]
module deepbook_lp_vaults::vault_test {
    use sui::test_scenario::{Self as test};
    use sui::sui::SUI;
    use sui::coin::{Self, Coin};
    use sui::test_utils;
    use deepbook_lp_vaults::vault::{Self, Vault};

    const USER1: address = @0xA;
    const USER2: address = @0xB;

    #[test]
    fun test_create_and_deposit_vault() {
        let mut scenario = test::begin(USER1);
        {
            let ctx = scenario.ctx();
            let vault_cap = vault::create_vault(0, ctx); // 0 for SUI
            test_utils::destroy(vault_cap);
        };

        scenario.next_tx(USER1);
        {
            let mut vault = scenario.take_shared<Vault>();
            let coins = coin::mint_for_testing<SUI>(1000, scenario.ctx());
            vault::deposit(&mut vault, coins, scenario.ctx());
            test::return_shared(vault);
        };

        scenario.next_tx(USER1);
        {
            let vault = scenario.take_shared<Vault>();
            assert!(vault::get_balance(&vault) == 1000, 0);
            test::return_shared(vault);
        };

        test::end(scenario);
    }

    #[test]
    fun test_withdraw_vault() {
        let mut scenario = test::begin(USER1);
        {
            let ctx = scenario.ctx();
            let vault_cap = vault::create_vault(0, ctx);
            test_utils::destroy(vault_cap);
        };

        scenario.next_tx(USER1);
        {
            let mut vault = scenario.take_shared<Vault>();
            let coins = coin::mint_for_testing<SUI>(1000, scenario.ctx());
            vault::deposit(&mut vault, coins, scenario.ctx());
            test::return_shared(vault);
        };

        scenario.next_tx(USER1);
        {
            let mut vault = scenario.take_shared<Vault>();
            vault::withdraw(&mut vault, 500, USER2, scenario.ctx());
            test::return_shared(vault);
        };

        scenario.next_tx(USER2);
        {
            let coins = test::take_from_sender<Coin<SUI>>(&scenario);
            assert!(coin::value(&coins) == 500, 0);
            test::return_to_sender(&scenario, coins);
        };

        scenario.next_tx(USER1);
        {
            let vault = scenario.take_shared<Vault>();
            assert!(vault::get_balance(&vault) == 500, 0);
            test::return_shared(vault);
        };

        test::end(scenario);
    }
}


