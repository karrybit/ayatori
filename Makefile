ENV=""
BASE_PATH=""
TOPIC=""
SUBSCRIPTION=""

build: ./target/release/ayatori-driver
	cargo build --release

help: build
	./target/release/ayatori-driver --help

run: build
	./target/release/ayatori-driver -e $(ENV) -b $(BASE_PATH) -t $(TOPIC) -s $(SUBSCRIPTION)

json: build
	./target/release/ayatori-driver -e $(ENV) -b $(BASE_PATH) -t $(TOPIC) -s $(SUBSCRIPTION) -f json

dot: build
	./target/release/ayatori-driver -e $(ENV) -b $(BASE_PATH) -t $(TOPIC) -s $(SUBSCRIPTION) -f dot | dot -Tsvg -o dependency.svg
