use common::{
    consensus_state::IConsensusState,
    ibc::Height,
    icon::icon::lightclient::v1::{ClientState, ConsensusState},
    traits::AnyTypes,
};
use cw_common::raw_types::Any;
use cw_light_client_common::ContractError;
use ics07_tendermint_cw::ics23::FakeInner;
use ics08_wasm::client_state::ClientState as WasmClientState;
use prost::Message;
use tendermint_proto::Protobuf;
pub fn get_consensus_state_key(height: Height) -> Vec<u8> {
    [
        "consensusStates/".to_string().into_bytes(),
        format!("{height}").into_bytes(),
    ]
    .concat()
}

pub fn get_client_state_key() -> Vec<u8> {
    "clientState".to_string().into_bytes()
}

pub fn to_wasm_client_state(
    client_state: ClientState,
    old_wasm_state: Vec<u8>,
) -> Result<Vec<u8>, ContractError> {
    use ibc::Height;
    let any = Any::decode(&*old_wasm_state).unwrap();
    let mut wasm_client_state =
        WasmClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(&any.value).unwrap();
    wasm_client_state.data = client_state.to_any().encode_to_vec();
    wasm_client_state.latest_height = Height::new(0, client_state.latest_height);
    let vec1 = wasm_client_state.to_any().encode_to_vec();
    Ok(vec1)
}

pub fn to_wasm_consensus_state(consensus_state: ConsensusState) -> Vec<u8> {
    let wasm_consensus_state = ics08_wasm::consensus_state::ConsensusState {
        data: consensus_state.to_any().encode_to_vec(),
        timestamp: consensus_state.timestamp().nanoseconds(),
        inner: Box::new(FakeInner),
    };
    wasm_consensus_state.to_any().encode_to_vec()
}

pub fn decode_client_state(data: &[u8]) -> Result<ClientState, ContractError> {
    let any = Any::decode(data).unwrap();
    let wasm_state =
        ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
            &any.value,
        )
        .unwrap();
    let any = Any::decode(&*wasm_state.data).unwrap();
    let state = ClientState::from_any(any).unwrap();
    Ok(state)
}

pub fn decode_consensus_state(value: &[u8]) -> Result<ConsensusState, ContractError> {
    let any = Any::decode(&mut &*value).unwrap();
    let wasm_consensus_state =
        ics08_wasm::consensus_state::ConsensusState::<FakeInner>::decode_vec(&any.value).unwrap();
    let any = Any::decode(&mut &wasm_consensus_state.data[..]).unwrap();
    let any_consensus_state = ConsensusState::from_any(any).unwrap();
    Ok(any_consensus_state)
}