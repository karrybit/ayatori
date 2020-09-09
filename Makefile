ENV="staging"
BASE_PATH=""
TOPIC=""
SUBSCRIPTION=""

build: ./target/release/ayatori-driver
	cargo build --release

json: build
	./target/release/ayatori-driver -e $(ENV) -b $(BASE_PATH) -f json -t $(TOPIC) -s $(SUBSCRIPTION)

dot: build
	./target/release/ayatori-driver -e $(ENV) -b $(BASE_PATH) -f dot -t $(TOPIC) -s $(SUBSCRIPTION) | dot -Tsvg -o dot.svg
