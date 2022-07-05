#!/bin/bash

cd $(dirname "$0")

cargo build --release

if [ $? -ne 0 ]
then
    exit $?
fi

mkdir -p /usr/local/search-router/
mv ./target/release/search-router /usr/local/search-router/
cp ./search-router.service /usr/local/search-router/
ln -s /usr/local/search-router/search-router.service /etc/systemd/system/multi-user.target.wants/search-router.service
systemctl enable search-router
systemctl restart search-router