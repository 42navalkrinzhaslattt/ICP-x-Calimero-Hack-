# meroctl

## Overview

Usage: `meroctl [OPTIONS] --node-name <NAME> <COMMAND>`

## Commands

- **app** - Command for managing applications (get, install, list)
- **context** - Command for managing contexts
- **identity** - Command for managing applications (generate)
- **proxy** - Command for managing proxy contract
- **call** - Executing read and write RPC calls
- **bootstrap** - Command for starting bootstrap
- **help** - Print this message or the help of the given subcommand(s)

### Global Options

- `--home <PATH>` - Directory for config and data [env: CALIMERO_HOME] [default: /Users/stefano/.calimero]
- `-n, --node-name <NAME>` - Name of node
- `--output-format <FORMAT>` - [default: plain-text] [possible values: json, plain-text]
- `-h, --help` - Print help
- `-V, --version` - Print version

### Environment Variables

- `CALIMERO_HOME` - Directory for config and data

## Command Details

### app

Command for managing applications

Usage: `meroctl --node-name <NAME> app <COMMAND>`

Commands:

- `get` - Fetch application details
- `install` - Install an application
- `list` - List installed applications
- `help` - Print help message

Examples:

List all applications:

```bash
meroctl --node-name node1 app ls
```

Sample output:

```json
{
  "id": "8YnK2NThDuosRC5MTpGXu1XEFTN5MpeMRWQt9cBGBVey",
  "size": 301893,
  "blobId": "6Vngc9bBQAMzEd2suqrHwWDGv5qPRMyaB3JU3rGTGAUT",
  "source": "file:///Users/stefano/Documents/Code/Hackathons/Calimero/hackathon/exp/hello-app/res/hello_app.wasm",
  "metadata": {}
}
```

Get application details:

```bash
meroctl --node-name node1 app get <APP_ID>
```

### context

Command for managing contexts

Usage: `meroctl --node-name <NAME> context <COMMAND>`

Commands:

- `list` - List all contexts
- `create` - Create a new context
- `join` - Join an application context
- `invite` - Create invitation to a context for an invitee
- `get` - Fetch details about the context
- `delete` - Delete a context
- `watch` - Watch events from a context
- `update` - Update app in context
- `help` - Print help message

#### Create Context Options

Usage: `meroctl context create [OPTIONS] --protocol <PROTOCOL> [METADATA]`

Options:

- `-a, --application-id <APPLICATION_ID>` - The application ID to attach to the context
- `-p, --params <PARAMS>` - Parameters to pass to the application initialization function
- `-w, --watch <WATCH>` - Path to the application file to watch and install locally
- `-s, --seed <CONTEXT_SEED>` - Seed for random generation of the context id
- `--protocol <PROTOCOL>` - Protocol to use
- `-h, --help` - Print help

Examples:

List contexts:

```bash
meroctl --node-name node1 context ls
```

Create context:

```bash
meroctl --node-name node1 context create --application-id <appId>
```

Create context in dev mode:

```bash
meroctl --node-name node1 context create --watch <path> -c <contextId>
```

### identity

Command for managing applications

Usage: `meroctl --node-name <NAME> identity <COMMAND>`

Commands:

- `generate` - Generate public/private key pair used for context identity
- `help` - Print help message

Example:

```bash
meroctl --node-name node1 identity generate
```

### proxy

Command for managing proxy contract

Usage: `meroctl --node-name <NAME> proxy <COMMAND>`

Commands:

- `get` - Fetch details about the proxy contract
- `help` - Print help message

#### Proxy Get Command

Usage: `meroctl proxy get <METHOD> <CONTEXT_ID> <PROPOSAL_ID>`

Arguments:

- `<METHOD>` - Method to fetch details [possible values: num-proposal-approvals, num-active-proposals, proposal, proposals, proposal-approvers]
- `<CONTEXT_ID>` - context_id of the context
- `<PROPOSAL_ID>` - proposal_id of the proposal

### call

Executing read and write RPC calls

Usage: `meroctl --node-name <NAME> call [OPTIONS] --as <EXECUTOR> <CONTEXT_ID> <METHOD>`

Arguments:

- `<CONTEXT_ID>` - ContextId of the context
- `<METHOD>` - Method to fetch details

Options:

- `--args <ARGS>` - JSON arguments to pass to the method
- `--as <EXECUTOR>` - Public key of the executor
- `--id <ID>` - Id of the JsonRpc execute call [default: dontcare]
- `-h, --help` - Print help

Example:

```bash
meroctl --node-name node1 call <CONTEXT_ID> <METHOD>
```
