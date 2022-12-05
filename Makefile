t.PHONY: all
all: tests

.PHONY: e2e-test
.SILENT: e2e-test
e2e-test: 
	cd e2e/tests && ./test.sh
	
.PHONY: unit-test
.SILENT: unit-test
 unit-test: 
	cd lib && cargo test


.PHONY: tests
.SILENT: tests
tests: e2e-test unit-test
				

