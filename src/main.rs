use digitrec_ai;
use digitrec_ai::Network;


fn main() {

    let neuro_netw = Network::new(vec![3, 2, 1]);

    digitrec_ai::start_app();

    // let dataset = digitrec_ai::get_dataset();
    // println!("{}", dataset[0][0][0].as_i64().unwrap() + 1);
}
