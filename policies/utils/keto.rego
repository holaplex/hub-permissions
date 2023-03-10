package hub.utils.keto
import input.keto
import future.keywords.if
import data.hub.graphql.lib.mutation_definitions
import data.hub.graphql.lib.mutation_arguments
import data.hub.graphql.lib.query_definitions
import data.hub.graphql.lib.query_arguments

get_namespace(x) = d {
  d := [d | 
    data.mappings[a].inputs[i] == x
    d := data.mappings[a].namespace
    ][0]
}

get_input_name(action) := t {
  t := [t |
   action == "view" 
   t := query_definitions[0].SelectionSet[0].Name
  ][0]
} 

get_input_name(action) := t {
  t := [t |
    action == "edit"
    t := mutation_definitions[0].VariableDefinitions[0].Type.NamedType
    ][0]
}

get_object_id(n) := id {
  id := input.graphql.variables[n]
} else := id {
  id := input.graphql.variables.input[n]
}

get_input_name(x) = d {
  d := [d | 
    mutation_definitions[a].VariableDefinitions[i].Type.NamedType == x
    d := mutation_definitions[a].VariableDefinitions[i].Variable
    ][0]
}

check_relation(x, action) := d { 
  input_name := get_input_name(action)
  namespace := get_namespace(input_name)
  object := get_object_id(lower(namespace))
  
  url_query := urlquery.encode_object({
      "namespace": namespace,
      "object": object,
      "relation": action,
      "subject_set.namespace": x.subject_set.namespace,
      "subject_set.object": x.subject_set.object,
      "subject_set.relation": x.subject_set.relation,
  })
  endpoint := concat("", [x.endpoint, "/relation-tuples/check?", url_query])
  res := http.send({
    "url": endpoint,
    "method":"GET",
    "headers": {
      "Content-Type": "application/x-www-form-urlencoded",
      "Accept": "application/json",
    },
    "force_json_decode": true,
    "force_cache": false,
    "force_cache_duration_seconds": 5,
    "timeout":"2s",
  })

  d := res.body.allowed
}

expand_relations(x) := d {
  url_query := urlquery.encode_object({
      "namespace": x.namespace,
      "object": x.object,
      "relation": x.relation,
  })
  endpoint := concat("", [x.endpoint, "/relation-tuples/expand?", url_query])
  res := http.send({
    "url": endpoint,
    "method":"GET",
    "headers": {
      "Content-Type": "application/x-www-form-urlencoded",
      "Accept": "application/json",
    },
    "force_json_decode": true,
    "force_cache": false,
    "force_cache_duration_seconds": 5,
    "timeout":"2s",
  })

  d := res.body.children
}
