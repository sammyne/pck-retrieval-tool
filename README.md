# Hello World example without CMake

## Quickstart

```bash
rm -rf build
mkdir build
cd build

cmake -DCMAKE_BUILD_TYPE=Prerelease ..
make run
```

Current output goes as 

```bash
[+] done new enclave: 2
thread 'main' panicked at 'generate quote: "[-] get target info: SGX_QL_NETWORK_ERROR"', src/main.rs:158:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Progress
- [x] Build the app with cargo only 
- [ ] Build the enclave with cargo only

## References
- Intel® Software Guard Extensions (Intel® SGX) Data Center Attestation Primitives: ECDSA Quote Library API - March, 2020
- [Platform Software Management with SGX quote helper daemon set]
- [Intel SGX Provisioning Certification Service for ECDSA Attestation]
- [Intel® Software Guard Extensions Data Center Attestation Primitives (Intel® SGX DCAP): A Quick Install Guide]

[Platform Software Management with SGX quote helper daemon set]: https://docs.microsoft.com/en-us/azure/confidential-computing/confidential-nodes-out-of-proc-attestation
[Intel SGX Provisioning Certification Service for ECDSA Attestation]: https://api.portal.trustedservices.intel.com/documentation#pcs-certificate-v3
[Intel® Software Guard Extensions Data Center Attestation Primitives (Intel® SGX DCAP): A Quick Install Guide]: https://software.intel.com/content/www/us/en/develop/articles/intel-software-guard-extensions-data-center-attestation-primitives-quick-install-guide.html
