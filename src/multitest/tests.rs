use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::App;

use crate::{
    errors::ContractError,
    multitest::proxy::SeiContractCodeId,
    responses::{GetBalanceResp, OwnerResp},
};

const SEI: &str = "sei";

#[test]
fn test_ownership() {
    let mut app = App::default();

    let owner = Addr::unchecked("addr0001");

    let code_id = SeiContractCodeId::store_code(&mut app);

    let contract = code_id.instantiate(&mut app, &owner, "app", None).unwrap();

    let resp = contract.get_owner(&app).unwrap();

    assert_eq!(
        resp,
        OwnerResp {
            owner: owner.to_string()
        }
    );
}

#[test]
fn test_transfers() {
    let owner = Addr::unchecked("addr0000");
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &owner, coins(10, SEI))
            .unwrap();
    });

    let code_id = SeiContractCodeId::store_code(&mut app);

    let contract = code_id
        .instantiate(&mut app, &owner, "transfer", None)
        .unwrap();

    // let err = contract
    //     .transfer(
    //         &mut app,
    //         &owner,
    //         "addr0001".to_string(),
    //         "addr0002".to_string(),
    //         &coins(15, SEI),
    //     )
    //     .unwrap_err();
    // assert!(matches!(err, ContractError::Std(StdError::Overflow { .. })));

    contract
        .transfer(
            &mut app,
            &owner,
            "addr0001".to_string(),
            "addr0002".to_string(),
            &coins(10, SEI),
        )
        .unwrap();

    assert_eq!(
        app.wrap().query_balance(owner, SEI).unwrap().amount.u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance(contract.addr(), SEI)
            .unwrap()
            .amount
            .u128(),
        10
    );
    assert_eq!(
        contract.get_balance(&app, "addr0001".to_string()).unwrap(),
        GetBalanceResp {
            addr: "addr0001".to_string(),
            balance: Uint128::from(5u128),
        }
    );
    assert_eq!(
        contract.get_balance(&app, "addr0002".to_string()).unwrap(),
        GetBalanceResp {
            addr: "addr0002".to_string(),
            balance: Uint128::from(5u128),
        }
    );
    // No coins
    assert_eq!(
        contract.get_balance(&app, "addr0003".to_string()).unwrap(),
        GetBalanceResp {
            addr: "addr0003".to_string(),
            balance: Uint128::from(0u128),
        }
    );
}

#[test]
fn test_withdraws() {
    let owner = Addr::unchecked("addr0000");
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &owner, coins(10, SEI))
            .unwrap();
    });

    let code_id = SeiContractCodeId::store_code(&mut app);

    let contract = code_id
        .instantiate(&mut app, &owner, "transfer", None)
        .unwrap();

    contract
        .transfer(
            &mut app,
            &owner,
            "addr0001".to_string(),
            "addr0002".to_string(),
            &coins(10, SEI),
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(contract.addr(), SEI)
            .unwrap()
            .amount
            .u128(),
        10
    );
    assert_eq!(
        contract.get_balance(&app, "addr0001".to_string()).unwrap(),
        GetBalanceResp {
            addr: "addr0001".to_string(),
            balance: Uint128::from(5u128),
        }
    );

    let sender = Addr::unchecked("addr0001");

    // Can't withdraw  more than we have
    let err = contract
        .withdraw(&mut app, &sender, Uint128::from(9999999u128))
        .unwrap_err();

    assert_eq!(
        err,
        ContractError::InsufficientBalance {
            sender: sender.clone()
        }
    );

    // Withdraw 0 coins
    let err = contract
        .withdraw(&mut app, &sender, Uint128::from(0u128))
        .unwrap_err();

    assert_eq!(err, ContractError::InvalidZeroAmount {});

    // Withdraw 2 coins, so user 1 has 3 left
    contract
        .withdraw(&mut app, &sender, Uint128::from(2u128))
        .unwrap();

    assert_eq!(
        contract.get_balance(&app, sender.to_string()).unwrap(),
        GetBalanceResp {
            addr: sender.to_string(),
            balance: Uint128::from(3u128),
        }
    );
    assert_eq!(
        app.wrap()
            .query_balance(contract.addr(), SEI)
            .unwrap()
            .amount
            .u128(),
        8
    );
}

#[test]
fn test_fees() {
    let owner = Addr::unchecked("addr0000");
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &owner, coins(5, SEI))
            .unwrap();
    });

    let code_id = SeiContractCodeId::store_code(&mut app);

    let contract = code_id
        .instantiate(&mut app, &owner, "transfer", None)
        .unwrap();

    contract
        .transfer(
            &mut app,
            &owner,
            "addr0001".to_string(),
            "addr0002".to_string(),
            &coins(5, SEI),
        )
        .unwrap();

    // Owner should have 5%2 coins as a withdrawable balance
    assert_eq!(
        contract.get_balance(&app, owner.to_string()).unwrap(),
        GetBalanceResp {
            addr: owner.to_string(),
            balance: Uint128::from(1u128),
        }
    );

    // Withdraw it
    contract
        .withdraw(&mut app, &owner, Uint128::from(1u128))
        .unwrap();

    assert_eq!(
        contract.get_balance(&app, owner.to_string()).unwrap(),
        GetBalanceResp {
            addr: owner.to_string(),
            balance: Uint128::from(0u128),
        }
    );
}
