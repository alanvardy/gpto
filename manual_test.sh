# Does not cover complete function

echo "== TESTING -p how much wood can a woodchuck chuck if the woodchuck is 5lb and the wood is oak ==" && \
cargo run -- -p how much wood can a woodchuck chuck if the woodchuck is 5lb and the wood is oak && \
echo "== TESTING -p sing me a song -s the end ==" && \
cargo run -- -p sing me a song -s the end && \
echo "== TESTING -p tell a great joke -t 0 ==" && \
cargo run -- -p tell a great joke -t 0 && \
echo "== TESTING -p tell a great joke -e ==" && \
cargo run -- -p tell a great joke -e && \
echo "== TESTING --models ==" && \
cargo run -- --models && \
echo "== TESTING -m code-davinci-002 ==" && \
cargo run -- -m code-davinci-002 -p language is elixir create a function that adds two numbers && \
echo ""
echo "== ======= =="
echo "== SUCCESS =="
echo "== ======= =="

