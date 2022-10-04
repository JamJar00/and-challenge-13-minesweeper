# Minesweeper
The `minesweeper(grid)` function can be called to solve the minesweeper board given the location of mines. An `X` denotes a mine, a `-` denotes no mine. The board must be square and should be fed in as a continuous stream left to right, then down the page rather than as a multidimensional collection.

Unit tests can be run using cargo. If you're not a rust user, then the following commands will run the unit tests:
```
# Install rust
brew install rustup
rustup-init

# Run tests
cargo test
```
