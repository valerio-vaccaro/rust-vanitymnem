use clap::{Arg, App};
use colored::*;

use std::str::FromStr;
use std::convert::TryInto;

use bip39::{Mnemonic, Language};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath, ChildNumber};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;

use std::time::Instant;

use regex::Regex;

use qrcode::QrCode;
use qrcode::render::unicode;


fn main() {
    let banner = "
    '##::::'##::::'###::::'##::: ##:'####:'########:'##:::'##:'##::::'##:'##::: ##:'########:'##::::'##:
     ##:::: ##:::'## ##::: ###:: ##:. ##::... ##..::. ##:'##:: ###::'###: ###:: ##: ##.....:: ###::'###:
     ##:::: ##::'##:. ##:: ####: ##:: ##::::: ##:::::. ####::: ####'####: ####: ##: ##::::::: ####'####:
     ##:::: ##:'##:::. ##: ## ## ##:: ##::::: ##::::::. ##:::: ## ### ##: ## ## ##: ######::: ## ### ##:
    . ##:: ##:: #########: ##. ####:: ##::::: ##::::::: ##:::: ##. #: ##: ##. ####: ##...:::: ##. #: ##:
    :. ## ##::: ##.... ##: ##:. ###:: ##::::: ##::::::: ##:::: ##:.:: ##: ##:. ###: ##::::::: ##:.:: ##:
    ::. ###:::: ##:::: ##: ##::. ##:'####:::: ##::::::: ##:::: ##:::: ##: ##::. ##: ########: ##:::: ##:
    :::...:::::..:::::..::..::::..::....:::::..::::::::..:::::..:::::..::..::::..::........::..:::::..::
                      VanityMnem - create your vanity mnemonics - 2020/2021 Valerio Vaccaro
                               https://github.com/valerio-vaccaro/rust-vanitymnem
    ";
    println!("{}", banner.green());
    let matches = App::new("VanityMnem")
        .version("0.0.1")
        .author("Valerio Vaccaro <VanityMnem@valeriovaccaro.it>")
        .about("Create a valid Bitcoin mnemonic with a vanity address in a specific derivation.")
        .arg(Arg::with_name("network")
                 .short("n")
                 .long("network")
                 .takes_value(true)
                 .help("main, test (default=test)"))
        .arg(Arg::with_name("pattern")
                 .short("p")
                 .long("pattern")
                 .takes_value(true)
                 .required(true)
                 .help("Regex for pattern"))
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
          .arg(Arg::with_name("twelve")
                   .short("t")
                   .long("twelve")
                   .help("Create a twelve words mnemonic (default is twenty-four words mnemonic)"))
        .get_matches();

    let mut network = matches.value_of("network").unwrap_or("test");
    if network != "main" {
        network = "test";
    }
    let pattern = matches.value_of("pattern").unwrap_or("vale");
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
    let twelve = matches.is_present("twelve");
    let secp = Secp256k1::new();

    struct Stats {
        n_mnemonics: i32,
        n_addresses: i32,
        not_found: bool,
        network: String,
        mnemonic: String,
        xpub: String,
        derivation: String,
        address: String,
    }

    let mut stats = Stats {
        n_mnemonics: 0,
        n_addresses: 0,
        not_found: true,
        network: network.to_string(),
        mnemonic: "".to_string(),
        xpub: "".to_string(),
        derivation: "".to_string(),
        address: "".to_string(),
    };

    let start = Instant::now();

    while stats.not_found {
        stats.n_mnemonics += 1;
        let mnemonic: Mnemonic;
        if twelve {
            let array: [u8; 16]  = rand::random();
            mnemonic =  Mnemonic::from_entropy_in(Language::English, &array).unwrap();
        } else {
            let array: [u8; 32]  = rand::random();
            mnemonic =  Mnemonic::from_entropy_in(Language::English, &array).unwrap();
        }
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
        let re = Regex::new(pattern).unwrap();

        // search in addresses
        for i in 0..n_children {
            stats.n_addresses += 1;
            let zero = ChildNumber::from_normal_idx(0).unwrap();
            let n = ChildNumber::from_normal_idx(i.try_into().unwrap()).unwrap();
            let public_key = xpub.derive_pub(&secp, &vec![zero, n]).unwrap().public_key;
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
            if re.is_match(&address.to_string()){
                stats.mnemonic = mnemonic.to_string();
                stats.xpub = xpub.to_string();
                stats.derivation = i.to_string();
                stats.address = address;
                stats.not_found = false;
                break;
            }
        }
        let delta = start.elapsed().as_secs();
        println!("stats: {} mnemonics, {} addresses in {} seconds", stats.n_mnemonics, stats.n_addresses, delta);
    }

    println!("Network {}", stats.network.yellow());
    println!("Mnemonic {}", stats.mnemonic.red());
    println!("XPub {}", stats.xpub.red());
    println!("Receiving address on derivation {}{}{} -> {}\n", derivation.red(), "/0/".red(), stats.derivation.red(), stats.address.to_string().green());
    let code = QrCode::new(&stats.address).unwrap();
    let image = code.render::<unicode::Dense1x2>()
       .dark_color(unicode::Dense1x2::Light)
       .light_color(unicode::Dense1x2::Dark)
       .build();
    println!("{}", image);
}
