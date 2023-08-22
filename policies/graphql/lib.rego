package hub.graphql.lib

import data.graphql.schema as graphql_schema
import future.keywords.in

graphql_document := input.graphql.query

ast := graphql.parse(graphql_document, graphql_schema)

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
		walk(mutation_definitions, [k, value])
		value.Arguments
		alias := value.Alias
	]
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
