extern crate jack;

mod effects;
mod overdrive;

use std::io;
use effects::{Effect, CtrlMsg};
use overdrive::Overdrive;

fn main() {
    let (client, _status) =
        jack::Client::new("rasta", jack::ClientOptions::NO_START_SERVER).unwrap();

    // register ports
    let in_b = client
        .register_port("guitar_in", jack::AudioIn::default())
        .unwrap();
    let mut out_a = client
        .register_port("rasta_out_l", jack::AudioOut::default())
        .unwrap();
    let mut out_b = client
        .register_port("rasta_out_r", jack::AudioOut::default())
        .unwrap();

    let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let out_a_p = out_a.as_mut_slice(ps);
        let out_b_p = out_b.as_mut_slice(ps);
        let in_b_p = in_b.as_slice(ps);

        // Use the overdrive to process samples
        let mut overdrive = Overdrive::new();
        overdrive.process_samples(in_b_p, out_a_p, out_b_p);

        jack::Control::Continue
    };
    let process = jack::ClosureProcessHandler::new(process_callback);
    let active_client = client.activate_async((), process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();

    active_client.deactivate().unwrap();
}
