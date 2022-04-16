use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Binary, from_binary, Response, Timestamp};
use cosmwasm_vm::testing::{
    execute, instantiate, mock_env, mock_info, query, MockApi, MockStorage,
};
use cosmwasm_vm::{from_slice, Instance, Storage};
use cw_satoshi_test::mock_query::{mock_instance, AppMockQuerier};
use cw_satoshi_test::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

static WASM: &[u8] = include_bytes!("../satoshi.wasm");
static CALLS: &[u8] = include_bytes!("../contract-calls.json");
const CREATOR: &str = "juno1hxkppd7spnvm5s86z2rfze5pndg9wwee8g9x6v";
const DESERIALIZATION_LIMIT: usize = 20_000;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TxCall {
    pub height: u64,
    pub time: Timestamp,
    pub msg: ExecuteMsg,
    pub sender: String,
}

fn setup() -> Instance<MockApi, MockStorage, AppMockQuerier> {
    let mut deps = mock_instance(WASM, &[]);
    let msg = InstantiateMsg { count: 0 };
    let info = mock_info(CREATOR, &[]);
    let mut env = mock_env();

    env.block.height = 2543598u64;
    env.block.time = Timestamp::from_nanos(1648938300000u64);

    let res: Response = instantiate(&mut deps, env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    deps
}

#[test]
fn execute_works() {
    let mut deps = setup();
    deps.with_storage(|s| {
        let rs = s.get(b"state").0.unwrap().unwrap();
        println!("init: {}", Binary(rs).to_base64());
        Ok(())
    }).unwrap();

    let tx_calls: Vec<TxCall> = from_binary(&Binary::from(CALLS)).unwrap();

    // env.contract.address = "".into();
    for tx in tx_calls.into_iter() {
        let mut env = mock_env();

        env.block.height = tx.height;
        env.block.time = tx.time;

        let info = mock_info(tx.sender.as_str(), &[]);
        let _r: Response = execute(&mut deps, env, info, tx.msg).unwrap();
    }

    let res = query(&mut deps, mock_env(), QueryMsg::GetCount {}).unwrap();
    let r: CountResponse = from_slice(&res, DESERIALIZATION_LIMIT).unwrap();
    println!("Count {}", r.count);

    deps.with_storage(|s| {
        let rs = s.get(b"state").0.unwrap().unwrap();
        println!("res: {}", Binary(rs).to_base64());
        Ok(())
    }).unwrap();
}
