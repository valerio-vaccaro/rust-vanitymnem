# Vanitymnem
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
```

Create addresses an public/private keys from a valid menmonic.

Hey this is a PoC! I take no responsibility if you lose your bitcoins using this
program! Be careful and use testnet!

This version of derive was written in Rust and based on the bitcoin and bip39
 libraries. Much of the code has been adapted from the examples found in related
 repositories and documentation.

This project was the excuse to learn Rust.

## Install and usage
You can compile the derive binary using cargo.

```
cargo build
```

If you want run the binary generate you can call it directly or via cargo.

```
cargo run --bin derive -- --help

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

### Examples

Generate 25 mnemonic addresses with default path using twelve words mnemonics on testnet.

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


Mnemonic: journey speed tent indoor canyon cave dragon vacant popular cash sting erase
Network: test Derivation: m/84'/0'/0'
XPub: tpubDDBjvHkBwEmJYRQP5txQfdT3XViqSvCJ8qyNg86tNhvFrMv7vaaTbWKBJLVEf7YQs8gqoJpZ3bc1rhoLgS73LrDNrXArLY2hbAYgz6WdUx1
XPriv: tprv8gVhmshwns5dexNbCFHpGDnvxUCuHb1PZYNbPc4axS7s1sfMJBksR1hK8EtR5ncDzUsMkr1vA4Dn6tzAShrZAEw1CX1EAPdKDRvdK3Zq4aP

Path   Address                                    PubKey                                                             PrivKey

/0/0   mpxaiztHymo8ddRHm8qrETHNDLpGWrsYpq         031507e6a95c72c6f9dedfd11f3956184b2a5d2c2d5c495bcb74949eed909cf23e cN3kjBU7HHYrATTMwgvAmBZ4iwDPkxdpLoT9wP5wNSU7sP5ZydYp
/0/1   mhhYgMezw6QpSpeLfon4YuzrUsrD8kb54e         025d1e4facea2cec67ad9f3b6c1a36f58f5282659b266166cd62384cee7c160575 cUWdf1FT1hXaP5wqBbcFADmErNb5hDSp654sGyqWRmzGNyisNdFM
/0/2   n4ZjGojsQayxUT9R3hdRrBMXPa7pnwjie5         0288b11a2fcbd9d4089177871d85030ebdfc835e5ddf6589e1efa745151a21d821 cV9hCaG6tzku2uivEhZbZNfmMpRMNiX6x2fd7uzm38aZzPNWShEa
/0/3   mwVCAj1vHverQrmKWVBZu39KytDpPBsuFr         03c2da5ddf88d11f70a48a0cea9853b636d3d6a0fdbaee020c6b5adcac69305dc5 cVAfX3jWV4mbYpxospU3Em3UF5hr6A4U2x25Sj5Sj6yJjPxQMYbc
/0/4   muxaoZcf5o1uEdnJTKzyhQSPz2Ygb5L2Uc         02385fc8ea4a0dd03a49b1406c04f3b92e11c1e002b02b9dbb145451e52bc7012b cNcWEsrn3uWFuxKRpGyyrxiinfm1AzZ4oNvuRsTsy9JNE83gKfy8
/0/5   n3yuFZH6oyPtrm6icuvCznyV8wLQNejGqS         029c96635e90aa9bf124982cc2cf1db48e769e117dd0d9cbd8ff7351d964b196bb cPUXpjmpaXgiGwsKUfXQ7GWAAzCapCcu5qCpKzPs7Je4pXvo8cQ7
/0/6   mwQfjjCE2DhoekS9uUwNfUnat5rDDpn5sj         0273a8d55e28fe92f962cd12f7d6fdce57aa7fa86e20b8fabd89c3211524e6cd8d cMapqDEqNiQR39PNR568RH9LAHyF19T55KVEDdnvuAejNbYoJ6V9
/0/7   mriQACxZ9upKUk2J2MnzCp93MAfgz7eB6D         02b535deaf14edc410af1513bae62594bf57de288bcf0a325ff1024b83809467e6 cSXubuFUesLEucnB97bH36wa2D5h9FBoTx5xodu2WYnTkbWiFgKi
/0/8   n1UP2rgabTU9r5RrUVqwCPrXFs96z22fUe         02a6468c55eb1ce5402afe797674f42363bb772641189cb071c440128516ee9830 cURqF4DRZQL8Wvmfhb8nC8rFtiLJxCbf367PwTbmmXh3MPCfxjWm
/0/9   myQNxGRXERfzgENu1t2DW2eyP5XLfgrggi         03e888acfc00de0af2f6c6603d308061fbb4a9016c1cbb512ab19f571e31cd890d cP2y4BaSHC7WhB7ux6GfyRWV8rChjjpHGPMjZ8iP66vzibGHy9zF
/0/10  mphiZFrgKBRQUqxCDGqbndd5WagckwxnMo         0390afd969aec36e72cbb5ccb085597adfaa2bdff78b7453c470ff6d6011e2aa05 cSASyCwReiJZJdpdynLHd49ECe3WbZvmdjECaaZLQb8NYFtj8Lto
/0/11  mvyd1Y1qda1EvUF5mv5PQGoke9RBPT4Qf9         0331278344537a8d62ee159e534c4a110e5d0419da32c4cb8e99ece916059b7bb6 cQ9PuqLcvBtxextVjvrKHrtMk4PC6WCCZJFV8goBaRYv4c1jLJU1
/0/12  mfxGZS4yeBSJPmMywvB2DyjjYsw9L6sj2u         032c0479b8954e0ee0a5285bccac80add4e07c4d7d916be4faf0a4a433aa624765 cQg4TXCsjo75NyTxwQn6YQeMe34yPFtZSri1uTB9ymrpN4VrqwUL
/0/13  n3vVCq34T3gMozfRnW8TWFS6Z3LvHfHGcw         03b4ed33b3a63520842274d9bf13f5f95b52db33decc601c5d048cc98f3b5e099d cMdLC8WRLyDVCsM4HbGPjfkWrVvbbA4vTcoFQbL6iDj3eYYe8got
/0/14  n13Ek3irZerk8GcqB6tHA7yrDAqEoUYf38         02bf54d90b03e65f57f805579f82ccb2e154cb57ff6056b4411b43f31969075cc7 cPN67muzgFez1hqecFT6k5tmvcuhtjVbKuTaGifBkCxErq9N7wyg
/0/15  msPhTMnTstqiZ1eYJtVcAppLwC8WBXR4BD         0220b0d8bd85452ef993a69b07fdae74b545db54670f7247ea13597f901c3a8062 cNpC42YP2i92KBxSVMHCDtGBwZjsrbBeXViNYXDasd52JMcez6a6
/0/16  mjpQEnkqvrqLCy57NixJUAAikEmA7SkiK2         03cc8f963c573d8866ddd3d10d608fa078897682da7e88ba5e092190cac10c6552 cSQEiERUTKyG9zXQjLfeYbWdHHWdxsdmsiuQvbyKWx7qLNc5Cp8R
/0/17  mzWPTJztHHPfEQ8QR2EfbZapVMEnM82Nx3         033e23966a5b484829eac7d3510c104227e001ae86e71c08e36124ad7552b4f492 cUq5UD73p7H232cZ3VG1SSi4NggxPHipuB1VGkiyyvFwBHnjmhZf
/0/18  moGnJWJSMHntSyEYANDR85RWnQg753rjs8         02545e595e17fdfd38c8af7dab7f2972ae48121be296e214960c87c9e7353aef00 cP3hbu9A6GypERPbquPtf4XoFDFNhmWajp7fLLmMAmpi4eh6omwV
/0/19  mxyx7ogVyiVRzhg3a3VrVNrU6NnWpHX1di         0319c074556a1f3d3e5ab1fd34e3df7e8f18efaea9f3ddcefe2ec5e5e6c358dc90 cRJxhRUsvKCymyQycqke1wR4wM1numn9suPhujtYGTfJsaE9fLAg
/0/20  mqg3NzUuAVz5ePvbGEtjPvSukJkwczuv19         02d63ab68b9e3787d1eabbaaeae42abdc1f92faf74610dc14de7f7566f4aaaa065 cUEomzpGnGdASTBYRYUKrQQeJ51YrxWJJq26mBMJQZWVhfP524mC
/0/21  msEeGzoBr6XGquKP7hJ7MDBkXbjivbzXbL         03eb1eb49cc601de733ccee8be3effe4a2ffb8e38dadfc2822a81f45c635a08196 cVSnpkTZUtH1TbViiGNurgkfXGL4gGsMS52xc2QMbeq4t3FocAhP
/0/22  mnhH1JqzYLGKALapZUcEu39RKPEPc9cw8w         02058201053b59537a788856f05bc035dad94c235bc39580b162c34c8ffbdf955b cVrpEgiZfH5Svhbg89jBRsKwYDqn6nqVESqEwyzTRWuh3eFZ2kdL
/0/23  n2U6rkGndA12YbjNR695Q8nWUr4HP5rcLe         029ad526c4618d6413de7d54b214592f97d69d6d60c0a6a9f42b325413694369f1 cSGmC5zGAw8e78rjwyv7jSHnvVbBYmdqYVW2KbgZVmSZPtjpSYrQ
/0/24  mzgwiAP5uT5uLTjcdoypGTtuNbiYTZ1xSF         02f69bb9ea8a1eb94e3f3fa02e42eadf570b15e7abcdddb1957a155e9eddd2a797 cTCR59mrzZgC6HCPSUtWfesAymgBiXSmP3sH5gbh5vMmQhXpkc59
```

Generate 10 legacy addresses with default path using twelve words mnemonics.

```
./derive -m "journey speed tent indoor canyon cave dragon vacant popular cash sting erase" -n main -c 10 -a legacy

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

Mnemonic: journey speed tent indoor canyon cave dragon vacant popular cash sting erase
Network: main Derivation: m/84'/0'/0'
XPub: xpub6Cp7Q3vWDxorGdDXdhyKTi7gCNdBTXhEbDP5Ka3k2oAN75FQrMjNT4eM4JQT8cbB5E9wBjJgWVmyAXJjHaFoTQm5nvRYf1NU1CxHDyjuhfe
XPriv: xprv9ypkzYPcPbFZ4994XgSK6aAweLnh44yPDzTUXBe8UTdPEGvGJpR7uGKsD4im5RDud3LakkQ9zhdye3SRKVWcMBfQfsnvW2uGJLBCsP7rRq6

Path   Address                                    PubKey                                                             PrivKey

/0/0   1ASdRwoKAkMsrWwg3ZsUQY53MMDZdhxV8Q         031507e6a95c72c6f9dedfd11f3956184b2a5d2c2d5c495bcb74949eed909cf23e KwgmGGUFrDrb11z6ZH73Ps416huz6WY8GmJgpxdRsKp7ce3BdMVM
/0/1   13BbPJa284yZfiAixEogiznXctFWJAbFut         025d1e4facea2cec67ad9f3b6c1a36f58f5282659b266166cd62384cee7c160575 L49eC6FbadqKDeUZoBo7nuGBE9Hg2mM822vQAZNzvfLG8EduJ2AL
/0/2   1Q3myketbZYhhLfoL8f42G9CXaX7sH3kss         0288b11a2fcbd9d4089177871d85030ebdfc835e5ddf6589e1efa745151a21d821 L4nhjfGFTw4dsUFerHkUC4Ahjb7wiGRQszXA1VYFY1vZjeH875H1
/0/3   1GyEsfvwUuDbdkHhnvDC57w17td7R6RK26         03c2da5ddf88d11f70a48a0cea9853b636d3d6a0fdbaee020c6b5adcac69305dc5 L4og48jf415LPPVYVQeusSYQcrQSRhxmxuscLJcwDzKJUeovKQ6H
/0/4   1FSdWWXgGmaeTXJgjm2bsVE582wyivt1C4         02385fc8ea4a0dd03a49b1406c04f3b92e11c1e002b02b9dbb145451e52bc7012b KxFWmxrvcqozkWrARsArVeDfASTbWYTNjLnSKT1NU2eMyNxvrGTz
/0/5   1PTwxWC7zwxe5ed6uLwqAsmAGwjhV5jmAR         029c96635e90aa9bf124982cc2cf1db48e769e117dd0d9cbd8ff7351d964b196bb Ky7YMpmy9TzT7WQ46FiGjx16YkuB9kXD1o4MDZwMcBz4Znt11EX3
/0/6   1GtiSg7FDCGYsdxYBuxzqZaG26FWHT2ofn         0273a8d55e28fe92f962cd12f7d6fdce57aa7fa86e20b8fabd89c3211524e6cd8d KwDqNJEywei9shv72fH13xeGY4fqLhMP1HLm7DLRQ3zj7rR7e5wQ
/0/7   1CCSs9saLtP4hdYgJnpcNtviVB4z5hawes         02b535deaf14edc410af1513bae62594bf57de288bcf0a325ff1024b83809467e6 L2Av8zFdDodykBJukhn9fnSWPynHUo67PuwVhDSX1S8TVrSoVJ7J
/0/8   1LxRjobbnS2u4xxEkvsZNUeCPsYQ5DPj5Z         02a6468c55eb1ce5402afe797674f42363bb772641189cb071c440128516ee9830 L44qn9Da8LdsMVJQKBKeppMCGV2uHkVxy3xvq39GGR336eA5pkVf
/0/9   1JtRfDLYRQEju7uHJK3qg7SeX5vdmgsBqL         03e888acfc00de0af2f6c6603d308061fbb4a9016c1cbb512ab19f571e31cd890d KxfybGaar8RFXjeeZgTYc71RWcuJ5HibCMDGSiFsazGzTrDoX3gZ
```

## Check

You can check generate addresses using [iancoleman bip39 tool](https://iancoleman.io/bip39/).
