#![allow(unused)]
use std::env;
use std::env::args;
use std::fs::File;
use std::ptr::null;
use std::sync::Arc;
use std::time::{Instant, SystemTime};

use rand::prelude::*;
use rand_distr::Normal;
use simplelog::{Config, LevelFilter, WriteLogger};
use threadpool::ThreadPool;

use crate::board::{print_bb, Board};
use crate::move_info::BISHOP_MASK;
use crate::move_tables::{find_magic, print_new_magics, ratt, MoveTables, B_BIT, R_BIT};
use crate::movegen::{
    gen_all_moves, gen_check_moves, gen_moves, is_in_check, is_legal_move, moved_into_check,
    sq_attacked,
};
use crate::moves::Move;
use crate::perft::{perft, perft_mt_root, perft_new_movegen, perftree_root};
use crate::search::iterative_deepening;
use crate::tt::{AtomicTT, SeqTT};
use crate::uci::Uci;
use crate::zorbist::{Zorb};

mod board;
mod eval;
mod move_info;
mod move_tables;
mod movegen;
mod moves;
mod opening_book;
mod perft;
mod search;
mod tt;
mod uci;
mod zorbist;
mod new_movegen;


fn main() {
    Zorb::init();

    if args().count() > 1 {
        do_perftree();
        return;
    }

    // debug();
    do_perf();
    //
    // ZORB.show();

    return;

    // set up logger
    let date_time = chrono::Local::now().format("%d%m%H%M%S").to_string();
    let mut filename = format!("/home/george/Documents/progs/rookandroll/logs/log-{date_time}.log");
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(filename).unwrap(),
    );

    let mut uci = Uci::new("george", "rustinator2");
    uci.start();

    // do_search();
    // do_perf();
    // do_perf_mt();
    // debug();
    // do_perftree();

    // print_bb(8796093022208);
}

fn debug() {
    let mut b = Board::new_fen("6r1/p4k2/8/1p1N4/2pKP2P/5R1N/6P1/R1B5 b - - 0 33");

    let moves = gen_moves(&b, true);
    moves.iter().for_each(|m| println!("{m}"));
}

fn do_search() {
    let board = Board::new();
    // let board = Board::new_fen("");
    let mut tt = SeqTT::new();
    let best_move = iterative_deepening(&board, &mut tt);
    println!("best move: {}", best_move.unwrap().as_uci_string());
}

fn do_perf() {
    let b = Board::new();
    // let b = Board::new_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
    let depth = 6;
    let start = Instant::now();
    let mc = perft(&b, depth);
    // let mc = perft_new_movegen(&b, depth);
    let stop = start.elapsed();
    println!(
        "Depth: {depth}\t\tMoves: {mc}\t\tTime: {}ms",
        stop.as_millis()
    );
}

fn do_perftree() {
    let args: Vec<String> = args().collect();
    let depth: usize = args[1].parse().unwrap();
    let fen = &args[2];
    let moves: Option<&String> = args.get(3);

    perftree_root(depth, fen, moves);
}

fn do_perf_mt() {
    let b = Board::new();
    let mt = MoveTables::new_arc();
    let depth = 6;
    let start = Instant::now();
    let mc = perft_mt_root(Arc::new(b), depth, 12);
    let stop = start.elapsed();
    println!(
        "Depth: {depth}\t\tMoves: {mc}\t\tTime: {}ms",
        stop.as_millis()
    );
}
