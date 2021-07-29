#!/usr/bin/env bash

#
## https://www.openshift.com/blog/the-hidden-dangers-of-terminating-namespaces
#

set -e

readonly usage="usage: $0 <namespace>"
readonly ns="${1?${usage}}"

kubectl api-resources --verbs=list --namespaced -o name | \
xargs -n 1 kubectl get --show-kind --ignore-not-found -n "$ns"
