mod utils;
use utils::{Payment, DebitData, process_payment};


fn main() {
    let some_payment = Payment::Cash(String::from("314214"));
    process_payment(some_payment);

    let credit = Payment::CreditCard(String::from("3162"), 132.);
    process_payment(credit);

    let debit_d = DebitData{card_number:String::from("312"), amount:123.};
    let debit = Payment::DebitCard(debit_d);
    process_payment(debit);

    let crypto = Payment::Crypto{account_id: String::from("13213321"), amount: 1323.};
    process_payment(crypto)
    // println!("{}",u8::from(255))
}