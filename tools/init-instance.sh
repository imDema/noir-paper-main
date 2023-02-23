#!/bin/bash
mkdir -p /home/ubuntu/.ssh/
echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPhr4VYdlqByzHny1M0J46TF4qPJRQdq1Ivp9VJT9t/O ubuntu@ip-172-31-44-56" >> ~/.ssh/authorized_keys
sudo apt-get update
sudo apt-get dist-upgrade -y
