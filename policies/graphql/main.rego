package hub.graphql.main

import data.hub.utils.helpers.is_valid
import data.hub.utils.keto.build_objects as keto
import future.keywords.if

default allow := false

allow if {
	valid_relations := {r | is_valid(keto[r])}
	count(keto) == count(valid_relations)
}

invalid := [sprintf("%v: %v", [relation.namespace, relation.object]) |
	relation := keto[_]
	not is_valid(relation)
]
