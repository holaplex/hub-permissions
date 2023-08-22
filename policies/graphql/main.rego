package hub.graphql.main

import data.hub.graphql.lib.selections
import data.hub.utils.helpers.is_valid
import data.hub.utils.keto.build_objects as keto
import future.keywords.if
import future.keywords.in

default allow := false

results := [x |
	obj := keto[_]
	x := is_valid(obj)
]

allow if {
	count(selections) == count([x | x := results[_]; x == true])
}

parsed := {
	"keto": keto,
	"selections": selections,
}
