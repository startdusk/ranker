.PHONY: codeline
codeline:
	@tokei .

.PHONY: test/integration
test/integration:
	@cargo test --lib -- tests::polls::test_polls_lifecycle --exact --nocapture

.PHONY: test 
test: fmt
	@cargo nextest run

.PHONY: fmt
fmt:
	@cargo fmt 
	@cargo fmt -- --check
	@cargo clippy --all-targets --all-features --tests --benches -- -D warnings

.PHONY: run
run:
	@cargo run --bin filebox-server

.PHONY: watch
watch: fmt
	@RUST_BACKTRACE=1 cargo watch -x run

build-dev:
	@docker-compose build 

up-dev: 	
	@docker-compose up

down-dev:
	@docker-compose down

clear-none-docker-image:
	@docker rm $(docker ps -a -q) 
	@docker rmi $(docker images -f "dangling=true" -q)