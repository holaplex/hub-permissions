#!/bin/bash
s3_bucket=stage-holaplex-hub-policies-mpw
namespace=default
schema_raw=$(python3 clean-schema.py schema.graphql) && \
echo "$schema_raw" && \
jq --arg data "$schema_raw" '.schema |= $data' policies/graphql/data.json > out.json && \
cp out.json policies/graphql/data.json && rm out.json && \
opa build -b policies && \
aws s3 cp bundle.tar.gz s3://"$s3_bucket"/bundle.tar.gz && \
kubectl rollout restart deploy/hub-permissions-opa -n "$namespace"
