# script-icp.sh

## Understanding Context Invitation Payload Encoding

When running a distributed Mero network, nodes communicate through a peer-to-peer (p2p) network. During this process, the context invitation payload is automatically encoded in base58 format when transmitted between nodes. The `meroctl context join` command expects to receive this base58-encoded payload, which it then decodes internally before processing.

However, in our tutorial setup, we're running all nodes on the same machine, bypassing the p2p network transmission. This means the invitation payload we generate with `meroctl context invite` isn't automatically encoded. If we try to use the raw payload directly with the join command, we'll receive an error because it expects a base58-encoded string.

After exploring various options for base58 encoding (including checking for native bash utilities and Homebrew packages), we found that the most reliable solution is to use the `bs58` command-line tool, which can be installed via cargo:

```bash
cargo install bs58-cli
```

Once installed, we can encode our invitation payloads before passing them to the join command. This encoding step simulates what would happen in a real distributed deployment where nodes communicate over the p2p network.

```

```
