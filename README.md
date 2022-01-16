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
  
┌───────────────────┬─────────────────────────────────────────┐
│ Author            ┆ Title                                   │
╞═══════════════════╪═════════════════════════════════════════╡
│ Metzger, Wolfgang ┆ Wer arbeitet auf der Baustelle?         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Henkel, Christine ┆ Tiere im Wald                           │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Kein Autor        ┆ Dinosaurier                             │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Egerkrans, Johan  ┆ Dinosaurier und andere Wesen der Urzeit │
└───────────────────┴─────────────────────────────────────────┘
```
