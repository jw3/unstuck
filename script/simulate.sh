#!/usr/bin/env bash

#
## https://www.openshift.com/blog/the-hidden-dangers-of-terminating-namespaces
#

set -e

readonly usage="usage: $0 <namespace>"
readonly ns="${1?${usage}}"

kubectl create ns "$ns"
kubectl apply -n "$ns" -f - << EOF
apiVersion: v1
kind: Secret
metadata:
  name: test-secret
  finalizers:
    - kubernetes.io/example-finalizer
stringData:
  sensitiveKey: sensitiveValue
EOF
kubectl delete ns "$ns" --wait=false
kubectl get ns "$ns"
