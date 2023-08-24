package hub.utils.helpers

import data.hub.graphql.lib.mutation_arguments
import data.hub.graphql.lib.query_arguments
import data.hub.graphql.lib.selections
import data.hub.utils.keto.valid_relation
import data.permission as authz
import future.keywords.if

headers := {lower(k): v | v := input.request.headers[k]}

subject_id := id if {
	id := headers["x-client-id"]
} else := id if {
	id := headers["x-user-id"]
}

object_id(s, p) := id if {
	input.graphql.operation == "query"
	var_name = object.get(query_arguments[s], p, null)
	id := input.graphql.variables[var_name]
} else := id if {
	id := object.get(query_arguments[s], p, null)
}

object_id(s, p) := id if {
	input.graphql.operation == "mutation"
	var_name := mutation_arguments[s][p[0]]
	id := object.get(input.graphql.variables, [var_name, p[1]], null)
	not id == null
} else := id if {
	id := object.get(mutation_arguments[s], [p[0], p[1]], null)
}

is_valid(obj) if {
	user_id := object_id("user", authz.user.object)
	obj.namespace == "User"
	obj.subject_id == user_id
} else if {
	authz[obj.selection].skip
} else if {
	valid_relation(obj)
}
