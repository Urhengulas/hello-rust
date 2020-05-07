#[derive(Debug)]
enum EuState {
	France,
	Italy,
	Germany,
	// --snip--
}

enum Coin {
	Cent,
	Cent50,
	Euro(EuState),
	Euro2,
}

fn value_in_cents(coin: Option<Coin>) -> Option<f64> {
	match coin {
		Some(c) => match c {
			Coin::Cent => Some(0.01),
			Coin::Cent50 => Some(0.5),
			Coin::Euro(state) => {
				println!("A 1€ coin from {:?}!", state);
				Some(1.0)
			}
			Coin::Euro2 => Some(2.0),
		},
		None => None,
	}
}

fn main() {
	dbg!(value_in_cents(Some(Coin::Euro(EuState::Germany))));
}
