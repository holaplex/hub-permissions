package hub.graphql.main

import future.keywords.in
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.query_fields
import data.hub.graphql.lib.query_arguments
import data.hub.graphql.lib.mutation_definitions
import data.hub.graphql.lib.mutation_fields
import data.hub.graphql.lib.mutation_arguments
import data.hub.graphql.lib.valid_schema
import data.hub.graphql.lib.valid_query
import data.hub.utils.keto.check_relation
import input.request.headers as http_headers
import future.keywords.if

default allow := false

default action := "view"
action := "edit" if input.graphql.operation == "mutation"


headers = {lower(k): v | http_headers[_]; v := http_headers[k]}

get_subject_id() := id {
  id := headers["x-client-id"]
} else := id {
  id := headers["x-user-id"]
}

subject_id := get_subject_id()

skip_authz {
  ## Skip if mutation found in data.no_authz_inputs
  data.no_authz_inputs[_] == mutation_definitions[_].VariableDefinitions[_].Type.NamedType
}

skip_authz {
  ## Skip if query found in data.no_authz_inputs
  data.no_authz_inputs[_] == query_definitions[_].SelectionSet[_].Name
}

skip_authz {
  ## subject is querying itself
  subject_id == input.graphql.variables[query_arguments.user.id]
}

keto_allowed if check_relation(subject_id, action) == true
keto_allowed if skip_authz

valid_graphql {
  valid_query
  valid_schema
}
allow {
  keto_allowed
  valid_graphql
}

reason := { 
  "headers": headers,
  "subject_id": subject_id,
  "graphql": input.graphql,
  "mutation": { 
    "definitions": mutation_definitions,
    "fields": mutation_fields,
    "arguments": mutation_arguments,
    },
  "query": { 
    "definitions": query_definitions, 
    "fields": query_fields,
    "arguments": query_arguments,
    }
  }
