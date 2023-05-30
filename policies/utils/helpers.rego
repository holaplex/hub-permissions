package hub.utils.helpers

import future.keywords.if
import input.request.headers as http_headers
import data.hub.graphql.lib.query_arguments
import data.hub.graphql.lib.mutation_arguments

headers = {lower(k): v | v := http_headers[k]}

get_subject_id() := id {
  id := headers["x-client-id"]
} else := id {
  id := headers["x-user-id"]
}

get_object_id(s, p) := id {
  input.graphql.operation == "query"
  var_name = object.get(query_arguments[s], p, null)
  id := input.graphql.variables[var_name]
} else := id {
  id := object.get(query_arguments[s], p, null)
}

get_object_id(s, p) := id {
  input.graphql.operation == "mutation"
  var_name := mutation_arguments[s][p[0]]
  id := object.get(input.graphql.variables, [var_name, p[1]], null)
} else := id {
  id := object.get(mutation_arguments[s][p[0]], p[1], null)
}
