use crate::{
    blockchain::{address::Address, validation::is_valid_address},
    p2p::P2PMessage,
    state::State,
    update::effect::{Effect, map_effect},
};

pub fn handle_add_transaction(
    state: State,
    recipient: &Address,
    send_amount: u64,
    fee: u64,
) -> (State, Effect) {
    if !is_valid_address(recipient) {
        info!("invalid recipient address: {}", recipient.der);
        return (state, Effect::None);
    }
    if let Some(transaction) = state.chain.generate_transaction(
        &state.address,
        recipient,
        send_amount,
        &state.secret_key,
        &state.transactions,
        fee,
    ) {
        let (state, changed) = state.add_transaction(&transaction);
        if changed {
            info!("added transaction: {:?}", transaction);
        } else {
            error!("failed to add transaction: {:?}", transaction);
        }
        (
            state,
            map_effect(
                || Effect::Broadcast(P2PMessage::ResponseTransactions(vec![transaction])),
                changed,
            ),
        )
    } else {
        (state, Effect::None)
    }
}
