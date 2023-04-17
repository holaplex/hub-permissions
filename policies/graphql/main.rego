package hub.graphql.main

import future.keywords.if
import future.keywords.in

import data.hub.utils.helpers.get_object_id
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.mutation_definitions
import data.hub.utils.keto.check_relation
import data.hub.utils.keto.build_objects as keto
import data.hub.graphql.lib.selections
import data.permission as permission

default allow := false

skip_authz {
  # Skip Authz only if All queries and mutations in the operation have skip: true mapped
  a := [v | 
    s := selections[_]
      v := permission[s].skip
      v == true
  ]
  count(selections) == count(a)
}

self_query {
  # Check there's only one value in query_definitions
  count(query_definitions) == 1
  # Check the value in query is exactly is 'user'
  query_definitions[_].SelectionSet[_].Alias == "user"
  # Check there's only one value in query_definitions[_].SelectionSet
  count(query_definitions[_].SelectionSet) == 1
  # Subject is querying itself
  user_id := get_object_id("user", permission.user.object)
  keto[_].subject_id == user_id
}

allow {
  # All queries and mutations in the operation must pass the relation check
  results := [x | obj := keto[_]; x := check_relation(obj); not is_null(x)]
  count(selections) == count([x | x := results[_]; x == true])
}

allow if skip_authz
allow if self_query
