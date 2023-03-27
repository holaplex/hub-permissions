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

get_selection() := n {
  n := query_definitions[0].SelectionSet[0].Alias
} else := n {
  n := mutation_definitions[0].SelectionSet[0].Alias
}

get_action(x) = d {
  d := [d | 
    data.mappings[a].actions[b].selections[i] == x
    d := data.mappings[a].actions[b].action
    ][0]
}

get_namespace(x) = d {
  d := [d | 
    data.mappings[a].actions[b].selections[i] == x
    d := data.mappings[a].namespace
    ][0]
}

get_object_id(s, n) := id {
  id := input.graphql.variables[n]
} else := id {
  id := input.graphql.variables.input[n]
}

get_object_id(s, n) := id {
  not input.graphql.variables[n]
  id := query_arguments[n].id
} else := id {
  not input.graphql.variables.input[n]
  id := query_arguments[s].input[n]
}

get_object_id(s, n) := id {
  not input.graphql.variables[n]
  id := mutation_arguments[n].id
} else := id {
  not input.graphql.variables.input[n]
  id := mutation_arguments[s].input[n]
}
