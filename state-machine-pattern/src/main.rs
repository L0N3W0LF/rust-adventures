// Bottle

struct Bottle {
    volume: usize,
}

// Machine

struct BottleFillingMachine<State> {
    bottle: Bottle,
    state: State,
}

// States

struct Waiting {
    time: std::time::Duration,
}

struct Filling {
    rate: usize,
}

struct Done;

// Transitions

impl BottleFillingMachine<Waiting> {
    fn new(bottle: Bottle) -> Self {
        BottleFillingMachine {
            bottle: bottle,
            state: Waiting {
                time: std::time::Duration::new(1, 0),
            }
        }
    }
}

impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine {
            bottle: val.bottle,
            state: Filling {
                rate: 25,
            }
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
        BottleFillingMachine {
            bottle: val.bottle,
            state: Done,
        }
    }
}

fn main() {
    let bottle_to_fill = Bottle { volume: 250 };
    let waiting = BottleFillingMachine::<Waiting>::new(bottle_to_fill);
    let filling = BottleFillingMachine::<Filling>::from(waiting);
    let done = BottleFillingMachine::<Done>::from(filling);
}
