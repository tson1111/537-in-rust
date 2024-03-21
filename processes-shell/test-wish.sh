#! /bin/bash
cargo build --quiet

if ! [[ -x target/debug/wish ]]; then
    echo "wish executable does not exist"
    exit 1
fi

mv target/debug/wish ./
../tester/run-tests.sh $*


