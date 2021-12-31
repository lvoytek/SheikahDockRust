# SheikahDock
An application dock based on the Sheikah Slate


## Build

### Install Dependencies

Install Rust/Cargo as seen [here](https://www.rust-lang.org/tools/install).

Install Libraries:

Debian-based (Ubuntu, Linux Mint, etc)
```bash
sudo apt install libgtk-4-dev libgtk-3-dev libssl-dev libatk1.0-dev 
```

Download and build
```bash
git clone https://github.com/lvoytek/SheikahDock.git
cd SheikahDock
cargo build --release
```

Run
```bash
./target/release/sheikah_dock
```


Thanks to [Hunter Paramore](https://www.figma.com/@hparamore) for the use of [Zelda UI kit](https://www.figma.com/community/file/965825767811358609) assets.
