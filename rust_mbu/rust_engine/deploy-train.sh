set -e

cross build --release --target x86_64-unknown-linux-gnu
echo "[+] Build complete"

rsync --update target/x86_64-unknown-linux-gnu/release/engine install@156.17.4.12:/home/bestiapodgrunwaldem/arena/engines/morphe_engine
echo "[+] Copy complete"
echo "[+] Done"
