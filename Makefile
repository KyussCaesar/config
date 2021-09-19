.PHONY: test

test:
	rp -- build run cargo test -- --nocapture
