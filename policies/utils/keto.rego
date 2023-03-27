package hub.utils.keto
import data.hub.utils.helpers.get_subject_id
import data.hub.utils.helpers.get_selection
import data.hub.utils.helpers.get_action
import data.hub.utils.helpers.get_namespace
import data.hub.utils.helpers.get_object_id

# Collect data from graphql query and request headers
build_object() := d {
  subject_id :=  get_subject_id()
  selection := get_selection()
  action := get_action(selection)
  namespace := get_namespace(selection)
  object := get_object_id(selection, lower(namespace))

  d := {
  "namespace": namespace,
  "object": object,
  "selection": selection,
  "action": action,
  "subject_id":  subject_id,
  "subject_ns": "User",
  }
}

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
