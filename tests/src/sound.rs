pub mod instrument;

mod pv_mod {
    pub fn pub_err() {
        println!("pv_err() called");
        super::instrument::clarinet();
    }
}

fn ake() {
    println!("COUCOU");
}

pub fn pv_mod_fn() {
    println!("accessed from pv_mod module");
    pv_mod::pub_err();
}

// fn test() {
//     sound::instrument::clarinet(); // will compile as `clarinet()` and its parent are public
//     sound::instrument::pv_drum(); // will not compile as `pv_drum()` is not public
//     sound::pv_mod::pub_err(); // will not compile as `pv_mod` is not public
// }