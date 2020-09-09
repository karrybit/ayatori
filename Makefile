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

eg_run: build
	./target/release/ayatori-driver -e develop -b example -t sns_topic.tf -s sns_subscription.tf

eg_json: build
	./target/release/ayatori-driver -e develop -b example -t sns_topic.tf -s sns_subscription.tf -f json

eg_dot: build
	./target/release/ayatori-driver -e develop -b example -t sns_topic.tf -s sns_subscription.tf -f dot | dot -Tsvg -o dependency.svg
