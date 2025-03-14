# One-time memory (OTM) using SGX

I learned about the concept of [one-time programs](https://blog.cryptographyengineering.com/2022/10/27/one-time-programs/) and one-time memory today. Since actual hardware tokens to support one-time memory seem to be hard to build, I thought a "cloud" one-time memory might be fun to build. We use trusted hardware, e.g., SGX, for even more assuredness that the memory is not leaked and can only be read once. Downside is, that you need an internet connection to access this.

# API

The API is straightforward enough. There is an endpoint to program the memory which returns a uuid.

```
curl -X POST -H "Content-Type: application/json" -d '{"v0": "bear", "v1": "toad"}' http://$IP:8080/program
```

Then there is an endpoint to read the memory

```
curl -X POST -H "Content-Type: application/json" -d '{"uuid": "7a89305e-78b4-40ce-b833-5306742b275e", "selector": "0"}' http://$IP:8080/read
```

In a typical scenario, someone would program the memory, collect the uuids and send them to whoever is supposed to consume them.

# TODO

- Use https and RA-TLS for attestation
- Check if Rust builds are reproducible
