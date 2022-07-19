#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Account {
    qr_code: String,
    address: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Payment {
    address: String,
    qr_code: String,
    amount: u8,
}