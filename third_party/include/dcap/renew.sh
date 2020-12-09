#!/bin/bash

# Vendoring headers is to avoid the messy include from /usr/include failing the build of the 
# trusted parts

headers=(
    qve_header 
    sgx_pce
    sgx_ql_lib_common
    sgx_ql_quote
    sgx_quote_3
)

fromDir=/usr/include
for h in ${headers[*]}; do
    cp ${fromDir}/$h.h .
done