pub mod core;
pub mod game;

use bevy::prelude::*;
use bevy_ggrs::SessionType;
use ggrs::{P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};
use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;

use crate::core::{EngineApp, EngineConfig, EngineGGRSConfig};
use crate::game::GameApp;

#[derive(StructOpt)]
struct CommandLineArgs {
    #[structopt(long)]
    port: u16,
    #[structopt(long)]
    players: Vec<String>,
    #[structopt(long, default_value = "2")]
    input_delay: usize,
    #[structopt(long, default_value = "12")]
    max_prediction_window: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CommandLineArgs::from_args();
    let mut session_builder = SessionBuilder::<EngineGGRSConfig>::new()
        .with_num_players(cmd.players.len())
        .with_input_delay(cmd.input_delay)
        .with_sparse_saving_mode(true)
        .with_max_prediction_window(cmd.max_prediction_window);

    for (player_handle, player_address) in cmd.players.iter().enumerate() {
        if player_address == "local" {
            session_builder = session_builder.add_player(PlayerType::Local, player_handle)?;
        } else {
            let remote_player_address: SocketAddr = player_address
                .parse()
                .expect("Invalid remote player address");
            session_builder = session_builder
                .add_player(PlayerType::Remote(remote_player_address), player_handle)?;
        }
    }

    let socket = UdpNonBlockingSocket::bind_to_port(cmd.port)?;
    let session = session_builder.start_p2p_session(socket)?;

    App::new()
        .insert_engine(EngineConfig::default())
        .insert_game()
        //
        .insert_resource(session)
        .insert_resource(SessionType::P2PSession)
        //
        .add_system(print_events_system)
        //
        .run();

    Ok(())
}

fn print_events_system(mut session: ResMut<P2PSession<EngineGGRSConfig>>) {
    for event in session.events() {
        println!("GGRS Event: {:?}", event);
    }
}
