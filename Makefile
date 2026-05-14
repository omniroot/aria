.PHONY: build-go publish-ts

build-go:
	cd packages/aria && go build -o ../../bin/aria

publish-ts:
	deno publish --project @pure-beach/aria ./packages/aria
	deno publish --project @pure-beach/aria-desktop ./packages/aria-desktop