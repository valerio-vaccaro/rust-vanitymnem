# Mnemonic utilities in Rus

## Vanitymnem
Create a valid Bitcoin mnemonic with a vanity address in a specific derivation.
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
[VanityMnem documentation](https://github.com/valerio-vaccaro/rust-vanitymnem/blob/main/VANITYMNEM.md)

## Derive
Create addresses an public/private keys from a valid menmonic.
```
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

derive 0.0.1
Valerio Vaccaro <derive@valeriovaccaro.it>


USAGE:
    derive [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>          Native_segwit, nested_segwit or legacy (default=native_segwit)
    -c, --children <children>        Check children derivations from 0 to this value (default=100).
    -d, --derivation <derivation>    Base derivation (default=m/84'/0'/0')
    -m, --mnemonic <mnemonic>        Mnemonic
    -n, --network <network>          main, test (default=test)
```
[Derive documentation](https://github.com/valerio-vaccaro/rust-vanitymnem/blob/main/DERIVE.md)

## SignMessage

```
```
[SignMessage documentation](https://github.com/valerio-vaccaro/rust-vanitymnem/blob/main/SIGNMESSAGE.md)
