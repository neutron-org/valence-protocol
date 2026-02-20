use cosmwasm_schema::write_api;

use valence_account_utils::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use valence_base_account::contract::MigrateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        migrate: MigrateMsg
    }
}
