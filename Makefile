start:
	cargo watch -x "run -p server"
test:
	chmod +x ./client/index.bash
	./client/index.bash