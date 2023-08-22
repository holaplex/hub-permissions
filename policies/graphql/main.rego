package hub.graphql.main

import data.hub.graphql.lib.selections
import data.hub.utils.helpers.decision
import data.hub.utils.keto.build_objects as keto
import future.keywords.if
import future.keywords.in

default allow := false

allow if {
	results := [x |
		obj := keto[_]
		x := decision(obj)
	]
	count(selections) == count([x | x := results[_]; x == true])
}
