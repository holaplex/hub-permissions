#!/bin/bash
s3_bucket=stage-holaplex-hub-policies
namespace=stage-hub
schema_raw=$(python3 clean-schema.py schema.graphql) && \
jq --arg data "$schema_raw" '.schema |= $data' policies/data.json > out.json && \
cp out.json policies/data.json && rm out.json && \
opa build -b policies && \
aws s3 cp bundle.tar.gz s3://"$s3_bucket"/bundle.tar.gz && \
kubectl rollout restart deploy/hub-permissions-opa -n "$namespace"
