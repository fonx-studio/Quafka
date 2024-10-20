include .env

print:
	@echo "Environment variables:"
	@echo "CERT_PATH: ${CERT_PATH}"	
	@echo "KEY_PATH: ${KEY_PATH}"

run:
	@echo "Running..."
	CERT_PATH=${CERT_PATH} KEY_PATH=${KEY_PATH} cargo run

test:
	CERT_PATH=${CERT_PATH}  cargo test