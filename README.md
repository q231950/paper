# Paper

A cli to manage your library account in _Stiftung Hamburger Öffentliche Bücherhallen_.

**To be clear, this repository is neither owned nor endorsed by _Stiftung Hamburger Öffentliche Bücherhallen_.**

## Usage

```
cargo run -- -u A123456789 -p 1234

✔ Pick an item · 👩🏻‍💼👨🏼‍💼 account

┌─────────────────┬───────────────────────────────┐
│ Name            ┆ Jane Doe                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Address         ┆ 123 Main St                   │
│                 ┆ Anytown, USA                  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Email           ┆ jane.doe@anymail.com          │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Membership      ┆ 15/08/2014 - 22/12/2021       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Annual Fee      ┆ € 40,00 (Erwachsene >27 Last) │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Account Balance ┆ € 0,00                        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Account Credit  ┆ € 5,00                        │
└─────────────────┴───────────────────────────────┘
? Pick an item ›
❯ 👩🏻‍💼👨🏼‍💼 account
  📚 loans
  ❓ help

✔ Pick an item · 📚 loans

┌───────────────────┬───────────────────────────────────────────────┬────────────┬───────────┬───────────────┐
│ Author            ┆ Title                                         ┆ Due Date   ┆ Renewable ┆ Shelf Mark    │
╞═══════════════════╪═══════════════════════════════════════════════╪════════════╪═══════════╪═══════════════╡
│ Metzger, Wolfgang ┆ Wer arbeitet auf der Baustelle?               ┆ 02/02/2022 ┆       yes ┆ M61 280 691 9 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Henkel, Christine ┆ Tiere im Wald                                 ┆ 02/02/2022 ┆       yes ┆ M61 274 846 9 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Egerkrans, Johan  ┆ Dinosaurier und andere Wesen der Urzeit       ┆ 02/02/2022 ┆       yes ┆ M62 112 017 1 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Jansson, Tove     ┆ Die Mumins - Geschichten aus dem Mumintal     ┆ 19/02/2022 ┆       yes ┆ M61 631 377 9 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Bailey, Ella      ┆ Ein Tag auf unserem blauen Planeten ... in de ┆ 19/02/2022 ┆       yes ┆ M61 631 203 7 │
└───────────────────┴───────────────────────────────────────────────┴────────────┴───────────┴───────────────┘
```

## Make Swift Package

The Swift Package is made of a binary package target which depends on a XCFramework containing the built Rust `paper` library and some Swift headers.

You don't need to do anything except build the XCFramework to get changes in the Rust library reflected in the Swift package:

`$ cd paper; make apple`

For Apple builds the cargo-swift crate is used. **A matching version for the pinned version of uniffi is required**. Please checkout https://github.com/antoniusnaumann/cargo-swift?tab=readme-ov-file#installing-for-a-different-uniffi-version to see what version you need to install using `cargo install cargo-swift@<version> -f`

The current version of cargo-swift compatible with uniffi is **0.9.0**. To install it, run:
`cargo install cargo-swift@^0.9.0`
