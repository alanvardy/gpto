# Does not cover complete function

echo "== TESTING -p how much wood can a woodchuck chuck if the woodchuck is 5lb and the wood is oak ==" && \
cargo run -- -p how much wood can a woodchuck chuck if the woodchuck is 5lb and the wood is oak && \
echo "== TESTING --models ==" && \
cargo run -- --models && \
echo ""
echo "== ======= =="
echo "== SUCCESS =="
echo "== ======= =="

