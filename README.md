# Nym-Zcash-gRPC Demo

Interacting with a Lightwalletd instance via gRPC over the mixnet. Demo-ed and tested using the [Zingo](https://github.com/zingolabs/zingolib?tab=readme-ov-file#zingo-cli) `zingo-cli` wallet.

## Overview
* `src/run_client.rs` passes CLI args to the `NymProxyClient`, which transports bytes between the CLI wallet and the `NymProxyServer` instance via the mixnet.
* `src/run_server.rs` runs the `NymProxyServer` which listens for incoming traffic from the mixnet, passes this upstream to a Lightwalletd instance, and sends responses back to the `NymProxyClient` (which then passes to the wallet the gRPC request orginated from) using [anonymous replies (**S**ingle **U**se **R**eply **B**locks)](https://nymtech.net/docs/architecture/traffic-flow.html#private-replies-using-surbs).
```
 ┌────────────────────┐                                                        
 │   Local Machine    │                                                        
 │   +-----------+    │                                                        
 │   | zcash-cli |    │                                                        
 │   +-----------+    │                                                        
 │         ^          │                                                        
 │         |          │                                    ┌─────────────────────────┐ 
 │         v          │                                    │       Remote Host       │ 
 │ +----------------+ │             +--------+             │    +----------------+   │ 
 │ | NymProxyClient | │ <---------> | Mixnet | <---------> │    | NymProxyServer |   │ 
 │ +----------------+ │             +--------+             │    +----------------+   │ 
 └────────────────────┘                                    │            ^            │ 
                                                           │            |            │ 
                                                           │            |            │ 
                                                           │            |            │ 
                                                           │ +----------v----------+ │ 
                                                           │ |  Lightwalletd node  | │ 
                                                           │ +---------------------+ │ 
                                                           └─────────────────────────┘ 
```
## Usage
```sh
Server
------
Usage: server [OPTIONS] --upstream-address <UPSTREAM_ADDRESS>

Options:
  -u, --upstream-address <UPSTREAM_ADDRESS>  Upstream address, ie lightwalletd address
  -c, --config-dir <CONFIG_DIR>              Config directory [default: /tmp/mixnet-client]
  -h, --help                                 Print help

Client
------
Usage: client [OPTIONS] --server-address <SERVER_ADDRESS>

Options:
      --close-timeout <CLOSE_TIMEOUT>    Send timeout in seconds [default: 10]
  -s, --server-address <SERVER_ADDRESS>  Mixnet server address
      --listen-address <LISTEN_ADDRESS>  Listen address [default: 127.0.0.1]
      --listen-port <LISTEN_PORT>        Listen port [default: 8080]
  -h, --help                             Print help
```

## Run
```sh
# Have zebra + lightwalletd instances running in background. For the moment you cannot run lightwalletd using the TLS certificate, as requests are coming to/from localhost and not the cert address.
# Since lightwalletd is only receiving and sending traffic to our Nym server instance on the same server, this is not too much of an issue for the moment as traffic is encrypted as it moves through the mixnet,
# and the lightwalletd instance is on the same remote host as the Nym server instance.

# Terminal window 1:
./target/release/server -u <UPSTREAM_ADDRESS> # e.g. "127.0.0.1:9067" the default lightwalletd listening address

# Terminal window 2:
./target/release/client -s <NYM_PROXY_SERVER_ADDR> --close-timeout 120

# Terminal window 3: send `zingo-cli` traffic to 127.0.0.1:8080 and interact as per usual
```
