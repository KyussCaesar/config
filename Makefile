.PHONY: test

test:
	rp $(RPARGS) -- build run cargo test -- --nocapture
