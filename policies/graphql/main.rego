package hub.graphql.main

import future.keywords.if
import future.keywords.in

import data.hub.graphql.lib.mutation_arguments
import data.hub.graphql.lib.mutation_definitions
import data.hub.graphql.lib.query_arguments
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.selections
import data.hub.utils.helpers.object_id
import data.hub.utils.keto.build_objects as keto
import data.hub.utils.keto.check_relation
import data.permission as authz

default allow := false

skip_authz if {
	# Skip Authz only if All queries and mutations in the operation have skip: true mapped
	a := [v |
		s := selections[_]
		v := authz[s].skip
		v == true
	]
	count(selections) == count(a)
}

self_query if {
	# Check there's only one value in query_definitions
	count(query_definitions) == 1

	# Check the value in query is exactly is 'user'
	query_definitions[_].SelectionSet[_].Alias == "user"

	# Check there's only one value in query_definitions[_].SelectionSet
	count(query_definitions[_].SelectionSet) == 1

	# Subject is querying itself
	user_id := object_id("user", authz.user.object)
	keto[_].subject_id == user_id
}

allow if {
	# All queries and mutations in the operation must pass the relation check
	results := [x | obj := keto[_]; x := check_relation(obj); not is_null(x)]
	count(selections) == count([x | x := results[_]; x == true])
	#        false
}

allow if skip_authz

allow if self_query
