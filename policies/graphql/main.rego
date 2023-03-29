package hub.graphql.main

import future.keywords.if
import future.keywords.in

import data.hub.utils.helpers.get_object_id
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.mutation_definitions
import data.hub.utils.keto.check_relation
import data.hub.utils.keto.build_objects as keto
import data.hub.graphql.lib.selections

default allow := false


skip_authz {
  # Skip Authz only if All queries and mutations in the operation have skip: true mapped
  a := [v | 
     s := selections[_]
       v := data[s].skip
       v == true
  ]
  count(selections) == count(a)
}

allow {
  # All queries and mutations in the operation must pass the relation check
  keto_all_true := [x | obj := keto[_]; x := check_relation(obj); not is_null(x)]
  count(selections) == count([x | x := keto_all_true[_]; x == true])
}

allow if skip_authz

status_code := 401
