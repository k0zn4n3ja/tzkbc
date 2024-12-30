# Toy Zero-Knowledge Blockhain
I've been watching Mr Robot so please enjoy this hacker aesthetic:

    ⠀⢠⣾⣿⣿⣗⣢⠀⠀⠀⠀⠀⠀⢤⣒⣿⣿⣷⣆⠀⠀
    ⠀⠋⠉⠉⠙⠻⣿⣷⡄⠀⠀⠀⡄⣾⣿⠛⠉⠉⠉⠃⠀
    ⠀⠀⢀⡠⢤⣠⣀⡹⡄⠀⠀⠀⡘⣁⣤⣠⠤⡀⠀⠀⠀
    ⠀⡤⢾⣿⣿⣿⣿⡿⠀⠀⠀⠀⠸⣿⣿⣿⣿⣾⠦⣄⠀
    ⠀⠀⠀⠀⠉⠈⠀⠀⣠⠀⠀⠀⣀⠀⠈⠈⠁⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⣀⡔⢻⠀⠀⠀⠙⠢⡀⠀⠀⠀⠀⠀⠀
    ⢘⡦⣤⠤⠒⠋⠘⢤⡀⣀⣀⣀⡨⠚⠉⠓⠠⣤⢤⡞⠀
    ⠀⢹⡜⢷⣄⠀⣀⣀⣾⡶⢶⣷⣄⣀⡀⢀⣴⢏⡾⠁⠀
    ⠀⠀⠹⡮⡛⠛⠛⢻⡿⠥⠤⡽⡿⠛⠛⠛⣣⡾⠁⠀⠀
    ⠀⠀⠀⠙⢄⠁⠀⠀⠈⣇⣀⡼⠃⠀⠀⢁⠞⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠉⢆⡀⠀⢸⣿⡇⠀⢀⠠⠂⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠈⠁⠸⢿⡿⠃⠋⠁⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀

## Folder Structure

<!-- TODO: fill in with files -->

    tzbk/
    ├── docs/
    ├── src/
    │   ├── main.rs
    │   ├── lib.rs
    │   ├── blockchain/
    │   ├── zk/
    │   ├── network/
    │   ├── utils/
    │   └── tests/
    └─────────────────────

## Starting the Blockchain

For now we're just running the blockchain locally, to do this for one node you can run:

```sh
cargo run -- --port 8000
```

Whenever you want to run subsequent nodes, add your nodes to the peers list: 

```sh
cargo run -- --port 8000 --peers 127.0.0.1:8001
```

At this stage we're not doing a gossip protocol or other peer discovery mechanisms - but we might!

