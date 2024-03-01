#!/bin/bash -e

ssh-keygen -f "/home/john/.ssh/known_hosts" -R "20.6.104.129"
scp -r -i ~/.ssh/accelerate.pem target/release/immutable-bank-ledger azureuser@20.6.104.129:~
scp -r -i ~/.ssh/accelerate.pem immutable-bank.service azureuser@20.6.104.129:~

ssh -i ~/.ssh/accelerate.pem azureuser@20.6.104.129 'bash -s' <<EOF
sudo cp -f ~/immutable-bank-ledger /usr/bin
sudo cp -f ~/immutable-bank.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl stop immutable-bank || true
sudo killall immutable-bank-ledger || true
sudo systemctl enable immutable-bank || true
sudo systemctl restart immutable-bank
EOF
