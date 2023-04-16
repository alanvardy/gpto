# Does not cover complete function

echo "== TESTING -p how much wood can a woodchuck chuck if the woodchuck is 5lb and the wood is oak ==" && \
cargo run -- -p how much wood can a woodchuck chuck if the woodchuck is 5lb and the wood is oak && \
echo "== TESTING -p sing me a song -s the end ==" && \
cargo run -- -p sing me a song -s the end && \
echo "== TESTING -p tell a great joke -t 0 ==" && \
cargo run -- -p tell a great joke -t 0 && \
echo "== TESTING -m gpt-3.5-turbo-0301 ==" && \
cargo run -- -m gpt-3.5-turbo-0301 -p language is elixir create a function that adds two numbers && \
echo "== TESTING -p with stdin ==" && \
echo "what is one plus one" | cargo run -- -xp && \
echo "== TESTING -c ==" && \
cargo run -- -c && \
echo "== TESTING -c Roleplay as Shakespeare ==" && \
cargo run -- -c Roleplay as Shakespeare && \
echo ""
echo "== ======= =="
echo "== SUCCESS =="
echo "== ======= =="

