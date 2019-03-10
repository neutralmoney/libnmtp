// Ed25519 utility functions
//
// Copyright (c) 2019 Neutral Money Developers

extern crate rand;
extern crate ed25519_dalek;

use rand::Rng;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;

let mut csprng: OsRng = OsRng::new().unwrap();
let keypair: Keypair = Keypair::generate(&mut csprng);


