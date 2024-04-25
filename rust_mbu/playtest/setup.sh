echo '[+] Installing latest morphebot'
cargo install --path "../rust_engine/engine/"

pushd c-chess-cli
echo '[+] Installing c-chess-cli (chess arena client)'
python3 make.py
popd

echo '[+] All done.'

