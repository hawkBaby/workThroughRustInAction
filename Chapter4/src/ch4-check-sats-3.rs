#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };
    // This code will not work as per the instructions in the manual as the return type and expected type are different things
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // //"waiting" ...
    // let sat_a = check_status(sat_a);
    // let sat_b = check_status(sat_b);
    // let sat_c = check_status(sat_c);
}
