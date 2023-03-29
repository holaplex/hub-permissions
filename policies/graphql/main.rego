package hub.graphql.main

import future.keywords.if
import future.keywords.in

import data.hub.graphql.lib.selections
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.mutation_definitions
import data.hub.utils.helpers.get_object_id
import data.hub.utils.keto.check_relation
import data.hub.utils.keto.build_objects as keto

default allow := false


self_query {
  # Check there's only one value in query_definitions
  count(query_definitions) == 1
  # Check the value in query is exactly is 'user'
  query_definitions[_].SelectionSet[_].Alias == "user"
  # Check there's only one value in query_definitions[_].SelectionSet
  count(query_definitions[_].SelectionSet) == 1
  # Subject is querying itself
  user_id := get_object_id("user", data.user.object)
  keto[_].subject_id == user_id
}

skip_authz {
  # Skip Authz only if All queries/mutations in the operation have skip: true mapped
  all_skips := [v | s := selections[_]
                   v := data[s].skip
                   v == true
              ]
  count(selections) == count(all_skips)
}


allow {
  # Check that all objects in the keto array pass the check_relation function
  keto_all_true := [check_relation(obj) | obj := keto[_]]
  count(keto_all_true) == count([x | x := keto_all_true[_]; x == true])
}


allow if self_query
allow if skip_authz

reason := { 
  "object": keto, 
  "graphql": input.graphql,
 } 
