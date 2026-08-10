# P2P Tests

This directory contains automated P2P inspection scripts. inspect.sh performs the following checks:

- Starts a docker-compose setup for testing and checks the node operating status (/health)
- Registers peers via POST /peer and verifies idempotency (duplicate additions)
- Validates mutual peer visibility (/peers)
- Confirms that node A has 1 or more blocks (≥1 block) before submitting transactions
- Issues a sample transaction to node A and verifies its propagation to node B (/chain)
- Executes an error-case addpeer against a non-existent IP to check error handling
- Collects node logs and saves the test results under results/

> [!CAUTION]
> Chain sync tests occasionally fail.
