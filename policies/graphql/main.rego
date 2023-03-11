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
import future.keywords.if
import input.keto

default allow := false

default action := "view"
action := "edit" if input.graphql.operation == "mutation"


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
  keto.subject_set.object == input.graphql.variables[query_arguments.user.id]
}


valid_graphql {
  valid_schema
  valid_query
}

keto_allowed if check_relation(keto, action) == true
keto_allowed if skip_authz

allow {
  valid_graphql
  keto_allowed
}

reason := { 
  "keto": keto,
  "headers": input.request.headers,
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
