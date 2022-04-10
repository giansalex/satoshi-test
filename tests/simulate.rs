use cosmwasm_std::{Binary, Response};
use cosmwasm_vm::testing::{
    execute, instantiate, mock_env, mock_info, query, MockApi, MockStorage,
};
use cosmwasm_vm::{from_slice, Instance, Storage};
use cw_satoshi_test::mock_query::{mock_instance, AppMockQuerier};
use cw_satoshi_test::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

static WASM: &[u8] = include_bytes!("../satoshi.wasm");
const CREATOR: &str = "juno188lvtzkvjjhgzrakha6qdg3zlvps3fz6m0s984e0wrnulq4px9zqhnleye";
const DESERIALIZATION_LIMIT: usize = 20_000;

fn setup() -> Instance<MockApi, MockStorage, AppMockQuerier> {
    let mut deps = mock_instance(WASM, &[]);
    let msg = InstantiateMsg { count: 0 };
    let info = mock_info(CREATOR, &[]);
    let res: Response = instantiate(&mut deps, mock_env(), info, msg).unwrap();
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

    let execute_msg = ExecuteMsg::Reset { count: 55 };
    let info = mock_info(CREATOR, &[]);
    let _r: Response = execute(&mut deps, mock_env(), info, execute_msg).unwrap();

    let execute_msg = ExecuteMsg::Reset { count: 42 };
    let info = mock_info(CREATOR, &[]);
    let _r: Response = execute(&mut deps, mock_env(), info, execute_msg).unwrap();

    // let d = deps.api();
    // deps.set_storage_readonly()
    let res = query(&mut deps, mock_env(), QueryMsg::GetCount {}).unwrap();
    let r: CountResponse = from_slice(&res, DESERIALIZATION_LIMIT).unwrap();
    println!("Count {}", r.count);

    deps.with_storage(|s| {
        let rs = s.get(b"state").0.unwrap().unwrap();
        println!("res: {}", Binary(rs).to_base64());
        Ok(())
    }).unwrap();
}
