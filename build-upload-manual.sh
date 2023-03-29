#!/bin/bash
s3_bucket=stage-holaplex-hub-policies-mpw
namespace=default
json_schema_path=policies/graphql/data.json
schema_raw=$(python3 clean-schema.py schema.graphql) && \
jq --null-input --arg data "$schema_raw" '.schema |= $data' > "$json_schema_path" && \
opa build -b policies && \
rm "$json_schema_path" && \
aws s3 cp bundle.tar.gz s3://"$s3_bucket"/bundle.tar.gz && \
kubectl rollout restart deploy/hub-permissions-opa -n "$namespace"
