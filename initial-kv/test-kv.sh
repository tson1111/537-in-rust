#! /bin/bash
cargo build --quiet

if ! [[ -x target/debug/kv ]]; then
    echo "kv executable does not exist"
    exit 1
fi

mv target/debug/kv .
../tester/run-tests.sh $*






