# port-forward-controller

WIP.

A port forward controller powered by PCP - a UPnP and NAT-PMP successor protocol
that almost any home router supports.

> This has nothing to do with `kubectl port-forward` kind of port forwarding
> and the corresponding APIs.

Run it in your Kubernetes cluster and manage dynamic port forwarding rules via
CRDs.

Provides a `LoadBalancer` implementation that automatically forwards port on
the router according to the `Service` configuration.

> A successor project for <https://github.com/MOZGIII/port-map-operator>.

## Ideas

- eBPF-powered PCP implementation for externally-managed
  VIP `LoadBalancer` services

  The idea there is we'd be able to send and receive the PCP packets -
  something we can't do by simply utilizing the usual networking at with
  a host-network `Pod`.

  This is, however, better fitted for the `LoadBalancer` service implementations
  themselves.

- non-PCP port forwarding

  Using the HTTP/Telnet/SSH for port forwarding via the admin panel of
  the router.

  This is much more flexible, allowing proper setup for passing traffic to
  an externally-managed L2 `LoadBalancer`s directly - but also to any arbitrary
  IP/port pair.
