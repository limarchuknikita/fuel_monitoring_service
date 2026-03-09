#!/usr/bin/env bash

for i in {1..10}; do
    echo "Send request $i to /ready endpoint"
    while true; do
        curl -s http://fuel-monitoring.local/ready > /dev/null
    done &
done

wait