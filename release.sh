#!/bin/bash -e

./build.sh

scp -i ~/.ssh/accelerate.pem target/release/immutable-bank-ledger azureuser@20.6.104.129:/usr/bin

ssh -i ~/.ssh/accelerate.pem azureuser@20.6.104.129 'bash -s' <<EOF
sudo cp -f ~/immutable-bank-ledger /usr/bin
EOF
