use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractState {
    contract_addr: String,
    field_name: Vec<String>,
    field_value: Vec<String>,
}

impl ContractState {
    pub fn new(
        contract_addr: String,
        field_name: Vec<String>,
        field_value: Vec<String>,
    ) -> ContractState {
        ContractState {
            contract_addr,
            field_name,
            field_value,
        }
    }

    pub fn get_contract_addr(&self) -> String {
        self.contract_addr.clone()
    }

    pub fn get_field_name(&self) -> &Vec<String> {
        &self.field_name
    }

    pub fn get_field_value(&self) -> &Vec<String> {
        &self.field_value
    }
}
