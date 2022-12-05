
t.PHONY: all
all: e2e

.PHONY: e2e
.SILENT: e2e
e2e: 
	cd e2e/tests && ./test.sh
	