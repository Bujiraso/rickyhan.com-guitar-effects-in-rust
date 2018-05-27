# About
These are running code samples for [Ricky Han's blog article on guitar effects](http://rickyhan.com/jekyll/update/2018/02/06/rust-guitar-pedal-effects-dsp.html)

# Example 1: Playback
1. Connect your input (guitar bass)
1. Ensure you do not hear audio output from playing your instrument
1. Boot up a Jack server
1. Ensure you do not hear audio output from playing
1. In the 01-playback directory, do `$ cargo run`
1. In jack (e.g. qjackctl Connect dialog), connect your capture output to the "rasta" application guitar sink, and connect the two "rasta" outputs to your system playback's
1. Play your input. You should now hear your instrument playing while the application is running
1. Hit enter when finished and ensure playback ceases on application close

This means you're successfully plugged into jack through Rust's bindings.
Congrats!

# License
Unknown. Contact rickyhan+blog@rickyhan.com
