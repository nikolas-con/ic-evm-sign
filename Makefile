
t.PHONY: all
all: e2e

.PHONY: e2e
.SILENT: e2e
e2e: 
	cd tests/e2e && ./test.sh
	