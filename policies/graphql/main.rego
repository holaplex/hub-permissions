package hub.graphql.main

import future.keywords.if
import future.keywords.in
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.query_fields
import data.hub.graphql.lib.query_arguments
import data.hub.graphql.lib.mutation_definitions
import data.hub.graphql.lib.mutation_fields
import data.hub.graphql.lib.mutation_arguments
import data.hub.utils.keto.check_relation
import data.hub.utils.keto.build_object as keto

default allow := false

skip_authz {
  ## Skip if mutation found in data.no_authz_inputs
  data.no_authz_selections[_] == query_definitions[0].SelectionSet[0].Alias
}

skip_authz {
  ## Skip if query found in data.no_authz_inputs
  data.no_authz_selections[_] == mutation_definitions[0].SelectionSet[0].Alias
}

skip_authz {
  ## subject is querying itself
  keto.subject_id == input.graphql.variables[query_arguments.user.id]
}

allow if check_relation(keto) == true
allow if skip_authz

reason := { 
  "keto": keto, 
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
