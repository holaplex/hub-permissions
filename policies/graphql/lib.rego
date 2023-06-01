package hub.graphql.lib

import future.keywords.in
import data.graphql.schema as graphql_schema
default graphql_document = {}

selections := s {
  input.graphql.operation == "query"
  s := [alias |
    alias := query_definitions[_].SelectionSet[_].Alias
  ]
} else = s {
  input.graphql.operation == "mutation"
  s := [alias |
    alias := mutation_definitions[_].SelectionSet[_].Alias
  ]
}


schema := s {
  s := graphql.parse_schema(graphql_schema)
}
query := q {
  q := graphql.parse_query(graphql_document)
}

ast := a {
  a := graphql.parse(graphql_document, graphql_schema)
}

valid_query := g {
  g := graphql.is_valid(graphql_document, graphql_schema)
}

valid_schema := g {
  g := graphql.schema_is_valid(graphql_schema)
}

graphql_document := g {
  g := input.graphql.query
}

known_types[t] {
  inline_fragments[_][t]
}

known_types[t] {
  t := query_fields[_][_]
}

query_types[t] := properties {
    t := known_types[_]
    frag_props := {p | p := inline_fragments[_][t][_]}
    field_props := {p |
      query_fields[_]["__type__"] = t
      query_fields[_][p]
      p != "__type__"}

    properties := {p:{}|  c := frag_props | field_props; p := c[_]}
}

inline_fragments[sub] {
  [_,node] := walk(query_definitions)
  node.TypeCondition
  sub := {type:fields |
    type := node.TypeCondition
    fields := [n | n := node.SelectionSet[_].Name]
  }
}

query_arguments := a {
  ast
  args := [v |
    count(query_definitions[i].SelectionSet[j].Arguments) > 0
    name := query_definitions[i].SelectionSet[j].Name

    args := {field: value |
      field := query_definitions[i].SelectionSet[j].Arguments[k].Name
      value := get_value(query_definitions[i].SelectionSet[j].Arguments[k].Value)
    }
    v := {name: args}
  ]
  a := {f: a | args[i][f]; a := {k: v | v := args[i][_][k]}}
}

mutation_arguments := a {
  ast
  args := [v |
    count(mutation_definitions[i].SelectionSet[j].Arguments) > 0
    name := mutation_definitions[i].SelectionSet[j].Name

    args := {field: value |
      field := mutation_definitions[i].SelectionSet[j].Arguments[k].Name
      value := get_value(mutation_definitions[i].SelectionSet[j].Arguments[k].Value)
    }
    v := {name: args}
  ]
  a := {f: a | args[i][f]; a := {k: v | v := args[i][_][k]}}
}

get_value(value) = value.Raw {
  value.Kind != 9
}

get_value(value) = children {
  value.Kind == 9
  children := {c.Name: c.Value.Raw | c := value.Children[_]}
  children != {}
}

query_definitions = d {
  ast
  d := [o |
    ast[a].Operations[i].Operation in ["query", "subscription"]
    o := ast[a].Operations[i]
    ]
}

mutation_definitions = d {
  ast
  d := [d |
    ast[a].Operations[i].Operation == "mutation"
    d := ast[a].Operations[i]
    ]
}

query_fields := fs {
  ast
  flds := [v |
    [_,node] := walk(query_definitions)

    sub := {{name:type} |
      name := node.SelectionSet[i].Name
      type := get_type_from_definition(node.SelectionSet[i].Definition)
      }
    count(sub) > 0

    v := {node.Name: (sub | {{"__type__":get_type_from_definition(node.Definition)}})}
  ]
  fs := {f:a | flds[_][f]; a := {k:v| v := flds[_][f][_][k]} }
}

mutation_fields := fs {
  ast
  flds := [v |

    [_,node] := walk(mutation_definitions)

    sub := {{name:type} |
      name := node.SelectionSet[i].Name
      type := get_type_from_definition(node.SelectionSet[i].Definition)
      }
    count(sub) > 0

    v := {node.Name: (sub | {{"__type__":get_type_from_definition(node.Definition)}})}
  ]
  fs := {f:a | flds[_][f]; a := {k:v| v := flds[_][f][_][k]} }
}

get_type_from_definition(definition) := t {
  t := definition.Type.Elem.NamedType
} else := t {
  t := definition.Type.NamedType
}
