ENVIRONMENT=staging
BASE_FILE_PATH="./test"
TOPIC_FILE_NAME="sns_topic.tf"
SUBSCRIBER_FILE_NAME="sns_sub.tf"

run:
	cargo build
	./target/debug/ayatori -e ${ENVIRONMENT} -b ${BASE_FILE_PATH} -t ${TOPIC_FILE_NAME} -s ${SUBSCRIBER_FILE_NAME}
