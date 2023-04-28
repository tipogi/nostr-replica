# Overview

![account-replica](./static/cover.png)

In Nostr, using a public-key cryptography system to authenticate each account, that scheme for some people might be a bit risky because they would not give the needed importance to the private key. If they lost the key, they would lost the owenership of the profile and since then, they would became a ghost 👻: Their past will exist but the present no.

This script does not secure your keys, for that there are different solutions: The aim of that is to create a backup of your account contact list. With the backup, you have the ability to replicate your lost account and not start following or messaging each peer explaining that you loose your keys.

## Commands

Prints message or the help of the given subcommand(s)

```bash
cargo run --help
```

Create a backup of your `followers` or the profiles that you are `following`. It will create as an output a JSON file. The __key__ that your are going to use, it is going to be the old one (👻)

```bash
# bech32_format_key will start npub
cargo run -- --backup --followers --following --key [bech32_format_key | hex_format_key]
```

With you new key pairs created add the new private key in the command. You can follow the previows `following` list and also direct message to all you followers explaining the reason of the change.

```bash
# bech32_format_key will start nsec
cargo run -- --account --nsec [bech32_format_key | hex_format_key]
```
