# Vanitymnem
```
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
```

Create a valid Bitcoin mnemonic with a vanity address in a specific derivation.

Hey this is a PoC! I take no responsibility if you lose your bitcoins using this
program! Be careful and use testnet!

This version of vanitymnem was written in Rust and based on the bitcoin and bip39
 libraries. Much of the code has been adapted from the examples found in related
 repositories and documentation.

This project was the excuse to learn Rust.

## Install and usage
You can compile the vanitymnem binary using cargo.

```
cargo build
```

If you want run the binary generate you can call it directly or via cargo.

```
cargo run --bin vanitymnem -- --help

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
VanityMnem 0.0.1

Valerio Vaccaro <VanityMnem@valeriovaccaro.it>
Create a valid Bitcoin mnemonic with a vanity address in a specific derivation.

USAGE:
    vanitymnem [FLAGS] [OPTIONS] --pattern <pattern>

FLAGS:
    -h, --help       Prints help information
    -t, --twelve     Create a twelve words mnemonic (default is twenty-four words mnemonic)
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>          Native_segwit, nested_segwit or legacy (default=native_segwit)
    -c, --children <children>        Check children derivations from 0 to this value (default=100).
    -d, --derivation <derivation>    Base derivation (default=m/84'/0'/0')
    -n, --network <network>          main, test (default=test)
    -p, --pattern <pattern>          Regex for pattern
```

### Examples

Generate a mnemonic with a derivation with beef (with upper or lower case chars)
in a testnet legacy address (check on first 5000 derivations).

```
cargo run --bin vanitymnem -- --pattern "[Bb][Ee][fF][fF]" -c 5000 -a legacy -n testnet

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
stats: 1 mnemonics, 5000 addresses in 3 seconds
stats: 2 mnemonics, 10000 addresses in 7 seconds
stats: 3 mnemonics, 15000 addresses in 11 seconds
stats: 4 mnemonics, 20000 addresses in 15 seconds
stats: 5 mnemonics, 25000 addresses in 19 seconds
stats: 6 mnemonics, 30000 addresses in 23 seconds
stats: 7 mnemonics, 35000 addresses in 27 seconds
stats: 8 mnemonics, 40000 addresses in 30 seconds
stats: 9 mnemonics, 45000 addresses in 34 seconds
stats: 10 mnemonics, 50000 addresses in 38 seconds
stats: 11 mnemonics, 55000 addresses in 42 seconds
stats: 12 mnemonics, 60000 addresses in 46 seconds
stats: 13 mnemonics, 65000 addresses in 51 seconds
stats: 14 mnemonics, 70000 addresses in 55 seconds
stats: 15 mnemonics, 75000 addresses in 59 seconds
stats: 16 mnemonics, 80000 addresses in 62 seconds
stats: 17 mnemonics, 83131 addresses in 65 seconds
Network test
Mnemonic now mango margin sense mango keen flame nephew gate two weather rain exist neither exhibit one rare twelve prefer asset robust fault boss move
XPub tpubDD1FdQzEZCwm6TMkGpv7iyMA9R1b2u6oyiSACvTwG2UXjM7CBYGYAtCox3dvtcWUzrkNYaXWSbfPJQkRxQ1iF6YciyjK62jrxopAxvk37nE
Receiving address on derivation m/84'/0'/0'/0/3130 -> msTMntuQzkuPTdQDFfJRQKbEFf53hegcho

█████████████████████████████████████
█████████████████████████████████████
████ ▄▄▄▄▄ █  ▄ █ ▄▄▀█▄▄▄█ ▄▄▄▄▄ ████
████ █   █ █▀▀█ ██▀▀▄██▀██ █   █ ████
████ █▄▄▄█ ███▀ ▀▀▄▀ ▄▀▀▀█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█ ▀ ▀ █▄▀ ▀ █ █▄▄▄▄▄▄▄████
████ ▀ ▄█ ▄▄▀▄▄█▀ ▀ ▄█ ▄ █▄▀▀ ▀ ▄████
████  ▀ █ ▄ █▀ ▄ ▄█▀▄█▄█▄▄▀▄█▄▄▄▀████
████▄ ▀  ▄▄▀█▀ ▄▄█ ▀▀ ██▄█▀▄▄▀▄▀█████
████▄▄ █▄▀▄██ ██▄▄▀▀▄▄  █ ▄ ▀ ▄▄ ████
████▄▀▀▀▀▀▄  ▀█  █  ██▀   ▀▀▄▄█▀█████
████▄█▀█▀▀▄  ▄ ▀█   ▄▀▀▀█ █ ▀▀▀▀█████
█████▄▄▄▄█▄█▀▄ ▀█▄▄█ ▄█▄ ▄▄▄ ▀▄ ▀████
████ ▄▄▄▄▄ █ ▄██▀▀ ▀▄█▀▄ █▄█ ▄▄▄█████
████ █   █ █▀▄ ▀▀▀█ ▀▄██▄ ▄  ▄▀▄▀████
████ █▄▄▄█ █▄ █ ▄▄█▀▀█▀███▄▄█▀█▀▄████
████▄▄▄▄▄▄▄█▄█▄▄█▄▄███▄▄▄██▄▄▄█▄█████
█████████████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

Generate a mnemonic with a derivation compatible with regex `[vV][aA][lL][eE]`
in a mainnet legacy address (check on first 1000 derivations), using twelve words
mnemonics.

```
./vanitymnem --pattern "[Vv][Aa][lL][eE]" -c 1000 -a legacy -t -n main

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

stats: 1 mnemonics, 1000 addresses in 0 seconds
stats: 2 mnemonics, 2000 addresses in 1 seconds
stats: 3 mnemonics, 3000 addresses in 2 seconds
stats: 4 mnemonics, 4000 addresses in 3 seconds
stats: 5 mnemonics, 5000 addresses in 4 seconds
stats: 6 mnemonics, 6000 addresses in 5 seconds
stats: 7 mnemonics, 7000 addresses in 5 seconds
stats: 8 mnemonics, 8000 addresses in 6 seconds
stats: 9 mnemonics, 9000 addresses in 7 seconds
stats: 10 mnemonics, 10000 addresses in 8 seconds
stats: 11 mnemonics, 11000 addresses in 9 seconds
stats: 12 mnemonics, 12000 addresses in 9 seconds
stats: 13 mnemonics, 13000 addresses in 10 seconds
stats: 14 mnemonics, 14000 addresses in 11 seconds
stats: 15 mnemonics, 15000 addresses in 12 seconds
stats: 16 mnemonics, 16000 addresses in 13 seconds
stats: 17 mnemonics, 17000 addresses in 14 seconds
stats: 18 mnemonics, 18000 addresses in 15 seconds
stats: 19 mnemonics, 19000 addresses in 15 seconds
stats: 20 mnemonics, 20000 addresses in 16 seconds
stats: 21 mnemonics, 21000 addresses in 17 seconds
stats: 22 mnemonics, 22000 addresses in 18 seconds
stats: 23 mnemonics, 23000 addresses in 19 seconds
stats: 24 mnemonics, 24000 addresses in 20 seconds
stats: 25 mnemonics, 25000 addresses in 20 seconds
stats: 26 mnemonics, 26000 addresses in 21 seconds
stats: 27 mnemonics, 27000 addresses in 22 seconds
stats: 28 mnemonics, 28000 addresses in 23 seconds
stats: 29 mnemonics, 29000 addresses in 24 seconds
stats: 30 mnemonics, 30000 addresses in 25 seconds
stats: 31 mnemonics, 31000 addresses in 25 seconds
stats: 32 mnemonics, 32000 addresses in 26 seconds
stats: 33 mnemonics, 33000 addresses in 27 seconds
stats: 34 mnemonics, 34000 addresses in 28 seconds
stats: 35 mnemonics, 35000 addresses in 29 seconds
stats: 36 mnemonics, 36000 addresses in 30 seconds
stats: 37 mnemonics, 37000 addresses in 30 seconds
stats: 38 mnemonics, 38000 addresses in 31 seconds
stats: 39 mnemonics, 39000 addresses in 32 seconds
stats: 40 mnemonics, 40000 addresses in 33 seconds
stats: 41 mnemonics, 41000 addresses in 34 seconds
stats: 42 mnemonics, 42000 addresses in 34 seconds
stats: 43 mnemonics, 43000 addresses in 35 seconds
stats: 44 mnemonics, 43788 addresses in 36 seconds
Network main
Mnemonic journey speed tent indoor canyon cave dragon vacant popular cash sting erase
XPub xpub6Cp7Q3vWDxorGdDXdhyKTi7gCNdBTXhEbDP5Ka3k2oAN75FQrMjNT4eM4JQT8cbB5E9wBjJgWVmyAXJjHaFoTQm5nvRYf1NU1CxHDyjuhfe
Receiving address on derivation m/84'/0'/0'/0/787 -> 1JNFjZTKgEjVruNcdwAaP5vALEDMFwuY6C

█████████████████████████████████████
█████████████████████████████████████
████ ▄▄▄▄▄ █▀ ▀▄▄▄ ▀▄█▀ ▀█ ▄▄▄▄▄ ████
████ █   █ ██▀█ █  ▀▄█▄▀▀█ █   █ ████
████ █▄▄▄█ █▄▀ ██▀▄██▀▄ ▄█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█▄█▄▀ ▀ ▀▄▀▄▀▄█▄▄▄▄▄▄▄████
████▄▀▄▀ ▀▄██▄▀▄▄█ ▀█  ▀▀█▀█▄██ █████
████▀▄ ▀▄▀▄  ▄▄▀█▄█ ▀█ ▀█▀ ████  ████
█████▄▄▄█ ▄▀█ ▀█▀▀█  ▀▀█ ▄ ▄▄▀▀ █████
███████▄ ▄▄█ ▄▀▄█▀ █  ▀▀█▄▄▀▀██  ████
█████ ██▀▀▄▀ █▄▀▄█ ▀▀▀▀▀  ▄██  ▀█████
████▄▀▄███▄ █ █▀█▄█▀▀▀▄█▀ ▀▄▄██▄▄████
████▄█▄███▄█ ▄▀ █▀█ ▀▄██ ▄▄▄ ▄▄█ ████
████ ▄▄▄▄▄ ██▄ ▄▄▀  ▀▄▀▄ █▄█ █▄█ ████
████ █   █ █▄ ▄ ██  ▀  ▀▄▄▄ ▄▀ ▄▄████
████ █▄▄▄█ █▄██▀  ▄▀█▀▀  ▀█▄ ▄█▀▄████
████▄▄▄▄▄▄▄█▄▄▄▄▄███▄█▄▄▄▄█▄▄██▄▄████
█████████████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

If you run this example your mnemonics will be different.

## Check

You can check generate address using [iancoleman bip39 tool](https://iancoleman.io/bip39/).
