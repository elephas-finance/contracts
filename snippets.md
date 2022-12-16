```sh
cargo wasm
```

```sh

sudo docker run --rm -v "$(pwd)":/contract   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   enigmampc/secret-contract-optimizer

```

```sh

secretcli tx compute store ./contract.wasm.gz --gas 5000000 --from wallet3 -y

```

```sh

secretcli query compute list-code

```

```sh

secretcli tx compute instantiate 18195 '{"name":"Sub NFT", "symbol":"SNFT","entropy":"eswar","config":{"public_token_supply":true,"enable_subscription":true},"subscription_info":{"frequency":864000,"rate":"1"}}' --from wallet3 --label gen041 -y

```

```sh

secretcli q compute tx 86D029659EBA396169A1DA8F575B6D3C6E887D54C9F0E41D3B5CDB724AC15C7E

```

```sh

secretcli q compute contract-hash secret10tdqgzc0wmldplnkjjpatelhsck84en4lvy8kp
```
