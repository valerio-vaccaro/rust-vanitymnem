use clap::{Arg, App};
use colored::*;

use std::str::FromStr;
use std::convert::TryInto;

use bip39::{Mnemonic};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath, ChildNumber};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;

fn main() {
    let banner = "
    ::::::::::::::::::::::::::::::::'########::'########:'########::'####:'##::::'##:'########:::::::::::::::::::::::::::::::::
    :::::::::::::::::::::::::::::::: ##.... ##: ##.....:: ##.... ##:. ##:: ##:::: ##: ##.....::::::::::::::::::::::::::::::::::
    :::::::::::::::::::::::::::::::: ##:::: ##: ##::::::: ##:::: ##:: ##:: ##:::: ##: ##:::::::::::::::::::::::::::::::::::::::
    :::::::::::::::::::::::::::::::: ##:::: ##: ######::: ########::: ##:: ##:::: ##: ######:::::::::::::::::::::::::::::::::::
    :::::::::::::::::::::::::::::::: ##:::: ##: ##...:::: ##.. ##:::: ##::. ##:: ##:: ##...::::::::::::::::::::::::::::::::::::
    :::::::::::::::::::::::::::::::: ##:::: ##: ##::::::: ##::. ##::: ##:::. ## ##::: ##:::::::::::::::::::::::::::::::::::::::
    :::::::::::::::::::::::::::::::: ########:: ########: ##:::. ##:'####:::. ###:::: ########:::::::::::::::::::::::::::::::::
    ::::::::::::::::::::::::::::::::........:::........::..:::::..::....:::::...:::::........::::::::::::::::::::::::::::::::::
    ::::::::::::::::             Derive - derive address from your mnemonics - 2021 Valerio Vaccaro            ::::::::::::::::
    ::::::::::::::::                   https://github.com/valerio-vaccaro/rust-vanitymnem                      ::::::::::::::::
    :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    ";
    println!("{}", banner.green());
    let matches = App::new("derive")
        .version("0.0.1")
        .author("Valerio Vaccaro <derive@valeriovaccaro.it>")
        .about("")
        .arg(Arg::with_name("mnemonic")
                 .short("m")
                 .long("mnemonic")
                 .takes_value(true)
                 .help("Mnemonic"))
        .arg(Arg::with_name("network")
                 .short("n")
                 .long("network")
                 .takes_value(true)
                 .help("main, test (default=test)"))
         .arg(Arg::with_name("derivation")
                  .short("d")
                  .long("derivation")
                  .takes_value(true)
                  .help("Base derivation (default=m/84'/0'/0')"))
          .arg(Arg::with_name("children")
                   .short("c")
                   .long("children")
                   .takes_value(true)
                   .help("Check children derivations from 0 to this value (default=100)."))
          .arg(Arg::with_name("address")
                   .short("a")
                   .long("address")
                   .takes_value(true)
                   .help("Native_segwit, nested_segwit or legacy (default=native_segwit)"))
        .get_matches();

    let menmonic_str = matches.value_of("mnemonic");
    let mut network = matches.value_of("network").unwrap_or("test");
    if network != "main" {
        network = "test";
    }
    let derivation = matches.value_of("derivation").unwrap_or("m/84'/0'/0'");
    let children = matches.value_of("children");
    let n_children;
    match children {
        None => n_children = 100,
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => n_children = n,
                Err(_) => n_children = 100,
            }
        }
    }
    let address_type = matches.value_of("address").unwrap_or("native_segwit");
    let secp = Secp256k1::new();
    let mnemonic: Mnemonic = Mnemonic::parse_normalized(menmonic_str.unwrap()).unwrap();
    let seed = mnemonic.to_seed_normalized("");
    let network_type: Network;
    if network == "main" {
        network_type = Network::Bitcoin;
    } else {
        network_type = Network::Testnet;
    }
    let root = ExtendedPrivKey::new_master(network_type, &seed).unwrap();
    let path = DerivationPath::from_str(derivation).unwrap();
    let child = root.derive_priv(&secp, &path).unwrap();
    let xpub = ExtendedPubKey::from_private(&secp, &child);

    println!("Mnemonic: {}\nNetwork: {} Derivation: {}\nXPub: {}\nXPriv: {}\n", mnemonic.to_string().red(), network.to_string().green(), path.to_string().yellow(), xpub.to_string().green(), child.to_string().red());
    println!("{:07}{:42} {:66} {:52}\n", "Path".to_string().yellow(), "Address".to_string().green(), "PubKey".to_string().yellow(), "PrivKey".to_string().red());
    for i in 0..n_children {
        let zero = ChildNumber::from_normal_idx(0).unwrap();
        let n = ChildNumber::from_normal_idx(i.try_into().unwrap()).unwrap();
        let public_key = xpub.derive_pub(&secp, &vec![zero, n]).unwrap().public_key;
        let private_key = child.derive_priv(&secp, &vec![zero, n]).unwrap().private_key;
        let address;
        if address_type == "nested_segwit" {
            address = Address::p2shwpkh(&public_key, network_type).unwrap().to_string();
        }
        else if address_type == "legacy" {
            address = Address::p2pkh(&public_key, network_type).to_string();
        }
        else{
            address = Address::p2wpkh(&public_key, network_type).unwrap().to_string();
        }
        println!("{:3}{:04}{:42} {:66} {:52}", "/0/".to_string().yellow(), i.to_string().yellow(), address.green(), public_key.to_string().yellow(), private_key.to_string().red());
    }

}
