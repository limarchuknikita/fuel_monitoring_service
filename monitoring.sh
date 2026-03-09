#!/usr/bin/env bash

while true; do
    clear
    kubectl get hpa -n fuel-monitoring
    sleep 1
done