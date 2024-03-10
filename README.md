# Solutions to Hakatemia
This repo is for random solutions to help solve [Hakatemia](https://hakatemia.fi) challenges.

## Crackme
Solution to the [crackme](https://www.hakatemia.fi/challenges/crackme)-challenge

### Building
`cargo build --bin crackme --release`

### Usage
```
Usage: crackme [OPTIONS] --target-ip <TARGET_IP>

Options:
  -t, --target-ip <TARGET_IP>                              Target IP address
  -s, --secret-message-port <SECRET_MESSAGE_PORT>          Secret message port [default: 6665]
  -e, --encryption-service-port <ENCRYPTION_SERVICE_PORT>  Encryption service port [default: 6666]
  -h, --help                                               Print help
  -V, --version                                            Print version
```

## tilinlukitus-dos-hyokkaykset
Solution to the [Tilinlukitus-DoS-hyökkäykset](https://www.hakatemia.fi/courses/salasanahyokkaykset/tilinlukitus-dos-hyokkaykset) lab 

### Building
Update `TARGET_URL` and `COOKIE` in `src/bin/tilinlukitus-dos-hyokkaykset.rs`.

`cargo build --bin tilinlukitus-dos-hyokkaykset --release`

### Usage
`./tilinlukitus-dos-hyokkaykset`
