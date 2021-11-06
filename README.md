# vanisol
A `Solana` Chain vanity address generator written in `Rust`.

## FYI
- This is a work in progress for learning purpose only.
- Official version: https://docs.solana.com/running-validator/validator-start#vanity-keypair
- You can use generated as token address here: https://spl.solana.com/token
  ```
  ## TOKEN_KEYPAIR : may be a keypair file or the ASK keyword. [default: randomly generated keypair]
  ## example: spl-token create-token -- id.json

  spl-token create-token -- TOKEN_KEYPAIR
  ```

## Usage
```shell
cargo run
cargo build --release
./target/release/vanisol
```

## TODO
- [ ] Add input parameters `.vanisol foo`
- [ ] Add input parameters `.vanisol --help`
- [ ] Add input parameters `.vanisol '^foo.\w+' --regxp`
