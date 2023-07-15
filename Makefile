all-tests: e2e-test unit-test

e2e-tests: 
	dfx start --clean --background; \
	dfx deploy; \
	cd e2e/tests && npm run test; \
	dfx stop
	
unit-tests: 
	cd lib && cargo test
