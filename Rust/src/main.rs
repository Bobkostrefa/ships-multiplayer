use std::net;
use client::Client;

mod client;
mod messages;

extern crate nalgebra as na;

fn main()
{
    let mut player = Client::new();
    player.get_input();
}