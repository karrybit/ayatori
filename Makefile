ENVIRONMENT=staging
BASE_FILE_PATH="./test"
PUBLISHER_FILE_NAME="sns_pub.tf"
SUBSCRIBER_FILE_NAME="sns_sub.tf"

run:
	cargo build
	./target/debug/ayatori -e ${ENVIRONMENT} -b ${BASE_FILE_PATH} -p ${PUBLISHER_FILE_NAME} -s ${SUBSCRIBER_FILE_NAME}
