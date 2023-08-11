package hub.graphql.lib

import data.graphql.schema as graphql_schema
import future.keywords.in

graphql_document := input.graphql.query

selections := s {
	input.graphql.operation == "query"
	s := [alias |
		walk(query_definitions, [k, value])
		value.Arguments
		alias := value.Alias
	]
} else = s {
	input.graphql.operation == "mutation"
	s := [alias |
		walk(query_definitions, [k, value])
		value.Arguments
		alias := value.Alias
	]
}

schema := graphql.parse_schema(graphql_schema)

query := graphql.parse_query(graphql_document)

ast := graphql.parse(graphql_document, graphql_schema)

valid_query := graphql.is_valid(graphql_document, graphql_schema)

valid_schema := graphql.schema_is_valid(graphql_schema)


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
		query_fields[_].__type__ = t
		query_fields[_][p]
		p != "__type__"
	}

	properties := {p: {} | c := frag_props | field_props; p := c[_]}
}

inline_fragments[sub] {
	[_, node] := walk(query_definitions)
	node.TypeCondition
	sub := {type: fields |
		type := node.TypeCondition
		fields := [n | n := node.SelectionSet[_].Name]
	}
}

query_arguments := a {
	ast
	top_level_args := {name: args |
		some i, j
		selection := query_definitions[i].SelectionSet[j]
		count(selection.Arguments) > 0
		name := selection.Alias
		args := {field: value |
			field := selection.Arguments[k].Name
			value := argument_value(selection.Arguments[k].Value)
		}
	}

	nested_args := {nfield: nvalue |
		some i, j, l
		selection := query_definitions[i].SelectionSet[j]
		nested_selection := selection.SelectionSet[l]
		count(nested_selection.Arguments) > 0
		nfield := nested_selection.Alias
		nvalue := {nested_selection.Arguments[m].Name: argument_value(nested_selection.Arguments[m].Value)}
	}

	a := object.union(top_level_args, nested_args)
}

mutation_arguments := a {
	ast
	top_level_args := {name: args |
		some i, j
		selection := mutation_definitions[i].SelectionSet[j]
		count(selection.Arguments) > 0
		name := selection.Alias
		args := {field: value |
			field := selection.Arguments[k].Name
			value := argument_value(selection.Arguments[k].Value)
		}
	}

	nested_args := {nfield: nvalue |
		some i, j, l
		selection := mutation_definitions[i].SelectionSet[j]
		nested_selection := selection.SelectionSet[l]
		count(nested_selection.Arguments) > 0
		nfield := nested_selection.Alias
		nvalue := {nested_selection.Arguments[m].Name: argument_value(nested_selection.Arguments[m].Value)}
	}

	a := object.union(top_level_args, nested_args)
}

argument_value(value) := value.Raw {
	value.Kind != 9
}

argument_value(value) := children {
	value.Kind == 9
	children := {c.Name: c.Value.Raw | c := value.Children[_]}
	children != {}
}

query_definitions := d {
	ast
	d := [o |
		ast[a].Operations[i].Operation in ["query", "subscription"]
		o := ast[a].Operations[i]
	]
}

mutation_definitions := d {
	ast
	d := [d |
		ast[a].Operations[i].Operation == "mutation"
		d := ast[a].Operations[i]
	]
}

query_fields := fs {
	ast
	flds := [v |
		[_, node] := walk(query_definitions)

		sub := {{name: type} |
			name := node.SelectionSet[i].Name
			type := definition_type(node.SelectionSet[i].Definition)
		}
		count(sub) > 0

		v := {node.Name: sub | {{"__type__": definition_type(node.Definition)}}}
	]
	fs := {f: a | flds[_][f]; a := {k: v | v := flds[_][f][_][k]}}
}

mutation_fields := fs {
	ast
	flds := [v |
		[_, node] := walk(mutation_definitions)

		sub := {{name: type} |
			name := node.SelectionSet[i].Name
			type := definition_type(node.SelectionSet[i].Definition)
		}
		count(sub) > 0

		v := {node.Name: sub | {{"__type__": definition_type(node.Definition)}}}
	]
	fs := {f: a | flds[_][f]; a := {k: v | v := flds[_][f][_][k]}}
}

definition_type(definition) := t {
	t := definition.Type.Elem.NamedType
} else := t {
	t := definition.Type.NamedType
}
