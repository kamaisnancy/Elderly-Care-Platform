# Decentralized Elderly Care and Health Management System

This project is a decentralized platform built on the Internet Computer for managing elderly care and health records. It allows users to register as elderly individuals, caregivers, or healthcare providers and manage user profiles, health records, medication reminders, and virtual consultations. The platform ensures robust access control and user management.

## Key Features

1. **User Management**
   - **Add User:** Allows users to create profiles as Elderly, Caregivers, or Healthcare Providers.
   - **Get All Users:** Retrieve a list of all user profiles.

2. **Health Record Management**
   - **Add Health Record:** Allows users to add health records to the system.
   - **Get All Health Records:** Retrieve a list of all health records available in the system.

3. **Medication Reminder Management**
   - **Create Medication Reminder:** Allows users to create medication reminders.
   - **Get All Medication Reminders:** Retrieve a list of all medication reminders.

4. **Virtual Consultation Management**
   - **Create Virtual Consultation:** Allows users to schedule virtual consultations.
   - **Get All Virtual Consultations:** Retrieve a list of all virtual consultations.

## Error Handling
   - **Not Found:** Returns an error if a requested item is not found.
   - **Unauthorized Access:** Returns an error if a user tries to perform an action without necessary permissions.


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```