use cosmwasm_std::{
    from_slice, Binary, Coin, ContractResult, Empty, QueryRequest, SystemError, SystemResult,
};
use cosmwasm_vm::internals::check_wasm;
use cosmwasm_vm::testing::{MockApi, MockInstanceOptions, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_vm::{Backend, BackendResult, GasInfo, Instance, InstanceOptions, Querier};

const DEFAULT_GAS_LIMIT: u64 = 4_000_000_000_000;

pub fn mock_instance(
    wasm: &[u8],
    contract_balance: &[Coin],
) -> Instance<MockApi, MockStorage, AppMockQuerier> {
    mock_instance_with_options(
        wasm,
        MockInstanceOptions {
            contract_balance: Some(contract_balance),
            gas_limit: DEFAULT_GAS_LIMIT,
            ..Default::default()
        },
    )
}

pub fn mock_instance_with_options(
    wasm: &[u8],
    options: MockInstanceOptions,
) -> Instance<MockApi, MockStorage, AppMockQuerier> {
    check_wasm(wasm, &options.supported_features).unwrap();
    let contract_address = MOCK_CONTRACT_ADDR;

    // merge balances
    let mut balances = options.balances.to_vec();
    if let Some(contract_balance) = options.contract_balance {
        // Remove old entry if exists
        if let Some(pos) = balances.iter().position(|item| item.0 == contract_address) {
            balances.remove(pos);
        }
        balances.push((contract_address, contract_balance));
    }

    let api = if let Some(backend_error) = options.backend_error {
        MockApi::new_failing(backend_error)
    } else {
        MockApi::default()
    };

    // let querier = MockQuerier::new(&balances);
    let backend = Backend {
        api,
        storage: MockStorage::default(),
        querier: AppMockQuerier {},
    };
    let memory_limit = options.memory_limit;
    let options = InstanceOptions {
        gas_limit: options.gas_limit,
        print_debug: options.print_debug,
    };
    Instance::from_code(wasm, backend, options, memory_limit).unwrap()
}

pub struct AppMockQuerier {}

impl Querier for AppMockQuerier {
    fn query_raw(
        &self,
        request: &[u8],
        _gas_limit: u64,
    ) -> BackendResult<SystemResult<ContractResult<Binary>>> {
        let request: QueryRequest<Empty> = from_slice(request).unwrap();

        let response = match request {
            QueryRequest::Stargate { path, data } => {
                SystemResult::Err(SystemError::UnsupportedRequest {
                    kind: format!("Error query path: {}, data: {}", path, data.to_base64()),
                })
            }
            _ => unimplemented!(),
        };

        (Ok(response), GasInfo::with_cost(1))
    }
}
