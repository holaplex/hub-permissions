package hub.utils.keto

import data.hub.graphql.lib.selections
import data.hub.utils.helpers.headers
import data.hub.utils.helpers.object_id
import data.hub.utils.helpers.subject_id
import data.permission as authz

build_objects := {[d |
	selection := selections[_]
	action := authz[selection].action
	namespace := authz[selection].namespace
	object := object_id(selection, authz[selection].object)

	d := {
		"namespace": namespace,
		"object": object,
		"selection": selection,
		"action": action,
		"subject_id": subject_id(),
		"subject_ns": "User",
	}
]}

check_relation(x) := d {
	url_query := urlquery.encode_object({
		"namespace": x.namespace,
		"object": x.object,
		"relation": x.action,
		"subject_set.namespace": x.subject_ns,
		"subject_set.object": x.subject_id,
		"subject_set.relation": "",
	})
	endpoint := concat("", [input.keto_endpoint, "/relation-tuples/check?", url_query])
	res := http.send({
		"url": endpoint,
		"method": "GET",
		"headers": {
			"Content-Type": "application/x-www-form-urlencoded",
			"Accept": "application/json",
			"X-Request-Id": headers["x-request-id"],
		},
		"force_json_decode": true,
		"force_cache": false,
		"force_cache_duration_seconds": 5,
		"timeout": "2s",
	})

	d := res.body.allowed
}
