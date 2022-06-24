prepare:
	rustup target add wasm32-unknown-unknown
build-contract-staking-dual-rewards:
	cargo build --release -p staking_dual_rewards --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/staking_dual_rewards.wasm 2>/dev/null | true
build-contract-staking-rewards-factory:
	cargo build --release -p staking_rewards_factory --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/staking_rewards_factory.wasm 2>/dev/null | true
build-erc20-contract:
	cargo build --release -p erc20 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20-token.wasm 2>/dev/null | true
build-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
test-only-staking-rewards-factory:
	cargo test -p staking_rewards_factory_tests
test-only-staking-dual-rewards:
	cargo test -p staking_dual_rewards_tests

copy-wasm-file-staking-dual-rewards:
	cp target/wasm32-unknown-unknown/release/*.wasm staking-dual-rewards/staking_dual_rewards_tests/wasm

copy-wasm-file-staking-rewards-factory:
	cp target/wasm32-unknown-unknown/release/*.wasm staking-dual-rewards-factory/staking_rewards_factory_tests/wasm

test-staking-rewards-factory: build-contract-staking-rewards-factory build-erc20-contract copy-wasm-file-staking-rewards-factory test-only-staking-rewards-factory

test-staking-dual-rewards: build-test-contract build-contract-staking-dual-rewards build-erc20-contract copy-wasm-file-staking-dual-rewards test-only-staking-dual-rewards

all:
	make test-staking-dual-rewards
	make test-staking-rewards-factory
clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf staking_rewards_factory_tests/wasm/*.wasm
	rm -rf staking_dual_rewards_tests/wasm/*.wasm

git-clean:
	git rm -rf --cached .
	git add .
