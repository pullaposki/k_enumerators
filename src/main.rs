enum ThingsInTheSky {
    Sun,
    Moon,
    Stars,
}

fn create_skystate(time:i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        5..=19 => ThingsInTheSky::Moon,
        _ => ThingsInTheSky::Stars,
    }
}

fn print_skystate(state:&ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("It's sunny!"),
        ThingsInTheSky::Moon => println!("It's moonlight!"),
        ThingsInTheSky::Stars => println!("It's starry!"),
    }
}

fn main() {
    let time = 8;
    let state = create_skystate(time);
    print_skystate(&state);

    let time = 20;
    let state = create_skystate(time);
    print_skystate(&state);
}
