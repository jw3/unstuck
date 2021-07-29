Fix a stuck namespace
===

For fixing a Kubernetes namespace stuck in terminating state.

https://www.openshift.com/blog/the-hidden-dangers-of-terminating-namespaces

### Options
1. [Fix](https://github.com/kubernetes/kubernetes/issues/60807#issuecomment-524772920)
2. [Force](https://github.com/kubernetes/kubernetes/issues/60807#issuecomment-408599873)

### Usage
```plain
unstuck 0.0.0
Unstick a stuck namespace

USAGE:
    unstuck <namespace> <action>

ARGS:
    <namespace>    The namespace
    <action>       The action to take (find, fix, force)
```



## Warranty
None
