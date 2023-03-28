package hub.utils.helpers
import input.request.headers as http_headers
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.query_arguments
import data.hub.graphql.lib.mutation_definitions
import data.hub.graphql.lib.mutation_arguments
import future.keywords.if

headers = {lower(k): v | v := http_headers[k]}

get_subject_id() := id {
  id := headers["x-client-id"]
} else := id {
  id := headers["x-user-id"]
}

get_object_id(s, p) := id {
  input.graphql.operation == "query"
  id := input.graphql.variables[query_arguments[s][p[0]]]
} else := id {
  id := object.get(query_arguments[s], p, null)
}

get_object_id(s, p) := id {
  input.graphql.operation == "mutation"
  id := input.graphql.variables[mutation_arguments[s][p[0]]][p[1]]
} else := id {
  id := object.get(mutation_arguments[s], p, null)
}


