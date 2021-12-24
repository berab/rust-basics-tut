pub enum Payment {
    Cash(String),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto{account_id: String, amount: f32}
} // just a list of items

pub struct DebitData {
    pub card_number: String,
    pub amount: f32,
}

pub fn process_payment(some_payment: Payment) {
   match some_payment {

       Payment::Cash(amt) => {
           println!("Payment with cash... {}", amt);
       }
       Payment::CreditCard(some_string, some_f32) => {
           println!("Paying with credit card. id: {}, amount: {}", some_string, some_f32);
       } // if only 2 are here, compile error will occur since not all optiosn are COVERED => pattern `DebitCard` not covered
       // Payment::DebitCard => {
       //     println!("Paying with debit card...")
       // }
       Payment::DebitCard(debit_data) => {
           println!("Paying with debit card. card number: {}, amount: {}", debit_data.card_number, debit_data.amount);
       }
       Payment::Crypto{account_id, amount} => {
           println!("Paying with debit card. account id: {}, amount: {}", account_id, amount);
       }
   }
}