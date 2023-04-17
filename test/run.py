from dataclasses import dataclass
from typing import Optional
import json
import requests

host = "127.0.0.1"
read_port = "4466"
write_port = "4467"


def write_url(path: str) -> str:
    return f"http://{host}:{write_port}/{path}"


def read_url(path: str) -> str:
    return f"http://{host}:{read_port}/{path}"


@dataclass
class SubjectSet:
    namespace: str
    object: str
    relation: Optional[str] = None


def create_relation_tuple(
    namespace: str,
    object: str,
    relation: str,
    subject_id: str = None,
    subject_set: SubjectSet = None
):
    print(f"Adding: {subject_set.namespace} {subject_set.object} to {relation} of {namespace} {object}")
    payload = {
        "namespace": namespace,
        "object": object,
        "relation": relation,
    }
    if subject_id is not None:
        payload["subject_id"] = subject_id
    if subject_set is not None:
        payload["subject_set"] = {
            "namespace": subject_set.namespace,
            "object": subject_set.object,
        }
        if subject_set.relation is not None:
            payload["subject_set"]["relation"] = subject_set.relation
    url = write_url("admin/relation-tuples")
    r = requests.put(url, data=json.dumps(payload))
    r.raise_for_status()


def check_relation_tuple(
    namespace: str,
    object: str,
    relation: str,
    subject_set: SubjectSet,
) -> bool:
    if relation != 'parents':
      q = f"Can {subject_set.namespace} {subject_set.object} {relation} {namespace} {object}?: "
    else:
      q = f"Is {subject_set.namespace} {subject_set.object} {relation} of {namespace} {object}?: "

    payload = {
        "namespace": namespace,
        "object": object,
        "relation": relation,
        "subject_set": {
            "namespace": subject_set.namespace,
            "object": subject_set.object,
            "relation": subject_set.relation,
        },
    }
    url = read_url("relation-tuples/check")
    r = requests.post(url, data=json.dumps(payload))
    if r.status_code != 200 and r.status_code != 403:
        r.raise_for_status()
    x = q, r.json()['allowed']
    print(x[0] + str(x[1]))
    return x



### 
print('------------------------------')
print("[.] Creating relations User -> Org")
print('------------------------------')
namespace = "Organization"
ss = SubjectSet(
    namespace='User',
    object='Alice',
    relation='session',
)

## Alice creates 2 organizations, Org1 and Org2, so its relation to both orgs is owners
relation = 'owners'
create_relation_tuple(namespace, 'Org1', relation, subject_set=ss)
create_relation_tuple(namespace, 'Org2', relation, subject_set=ss)


## Alice adds Bob with the viewer role to Org1.
## The relation of viewer is directly with the Org, so Bob will be able to see all Projects from the Organization.
relation = 'viewers'
ss.object = 'Bob'
create_relation_tuple(namespace, 'Org1', relation, subject_set=ss)

## John is a new user, unrelated to Alice or Bob.
## John creates a different org: JohnOrg, and adds a project into the org, JohnProject
relation = 'owners'
ss.object = 'John'
create_relation_tuple(namespace, 'JohnOrg', relation, subject_set=ss)

## John adds Anna as the owner of JohnProject
ss.object = 'Anna'
create_relation_tuple('Project', 'JohnProject', relation, subject_set=ss)

## Alice generates an API Token on the organization Org1 with the editors role.
## The relation of viewer is directly with the Org, so Bob will be able to see all Projects from the Organization.
relation = 'editors'
ss.object = 'api_token'
ss.relation = 'oauth2'
create_relation_tuple(namespace, 'Org1', relation, subject_set=ss)

print('------------------------------')
print("[.] Creating parent relations Org <- Project")
print('------------------------------')
## Alice creates project 'Project1' under 'Org1', so all members of Org1 will inherit the same permissions in this project. 
## The relation of Project1 to Org1 is 'parents'.
create_relation_tuple("Project", "Project1", "parents", subject_set=SubjectSet(
    namespace="Organization",
    object="Org1",
))

create_relation_tuple('Project', 'JohnProject', 'parents', subject_set=SubjectSet(
    namespace="Organization",
    object="JohnOrg",
))

print('------------------------------')
print("[.] Creating parent relations Org <- Project <- Drop")
print('------------------------------')
## Alice creates a drop in project 'Project1' under 'Org1'
create_relation_tuple("Drop", "Drop1", "parents", subject_set=SubjectSet(
    namespace="Project",
    object="Project1",
))

print('------------------------------')
print("[.] Creating parent relations  Drop <- Mint ")
print('------------------------------')
## Alice creates a drop in project 'Project1' under 'Org1'
create_relation_tuple("Mint", "Mint1", "parents", subject_set=SubjectSet(
    namespace="Drop",
    object="Drop1",
))

## Testing permissions
print('------------------------------')
print("[.] Testing Permissions: User -> Action -> Mint")
print('------------------------------')

ss = SubjectSet(
    namespace='User',
    object='Bob',
    relation='session',
)
namespace = "Mint"
object = "Mint1"

## Can User Bob transferAsset / retryMint from Org1/Project1/Drop1 ?
action = 'edit' 
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False


## Can User Alice transferAsset / retryMint from Org1/Project1/Drop1 ?
ss = SubjectSet(
    namespace='User',
    object='Alice',
    relation='session',
)
namespace = "Mint"
object = "Mint1"

action = 'edit' 
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Testing permissions
print('------------------------------')
print("[.] Testing Permissions: User -> Action -> Drop")
print('------------------------------')

ss = SubjectSet(
    namespace='User',
    object='Bob',
    relation='session',
)
namespace = "Drop"
object = "Drop1"

## Can User Bob edit Project1/Drop1?
action = 'edit'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Testing permissions
print('------------------------------')
print("[.] Testing Permissions: User -> Action -> Project")
print('------------------------------')

ss = SubjectSet(
    namespace='User',
    object='Bob',
    relation='session',
)
namespace = "Project"
object = "Project1"

## Can User Bob view Project Project1?
action = 'view'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User John view Project Project1?
ss.object = 'John'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Alice view Project Project1?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Bob delete Project Project1?
action = 'delete'
ss.object = 'Bob'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Alice delete Project Project1?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Alice edit Project Project1?
ss.object = 'Alice'
action = 'edit'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User John delete Project JohnProject?
ss.object = 'John'
action = 'delete'
object = 'JohnProject'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Anna delete Project JohnProject?
ss.object = 'Anna'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Alice delete Project JohnProject?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Alice view Project JohnProject?
ss.object = 'Alice'
action = 'view'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User api_token view Project JohnProject?
ss.object = 'api_token'
ss.relation = 'oauth2'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

print('------------------------------')
print("[.] Testing Permissions: User -> Action -> Organization")
print('------------------------------')

## Can User Alice view Organization Org1?
ss.object = 'Alice'
action = 'view'
namespace = 'Organization'
ss.relation = 'session'
object = 'Org1'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Bob view Organization Org1?
ss.object = 'Bob'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User John view Organization Org1?
ss.object = 'John'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User api_token view Organization Org1?
ss.object = 'api_token'
ss.relation = 'oauth2'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User api_token delete Organization Org1?
action = 'delete'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User api_token invite members to Organization Org1?
action = 'invite'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User John invite people to Organization Org1?
ss.object = 'John'
ss.relation = 'session'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Alice invite members to Org1?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Bob invite members to Org1?
ss.object = 'Bob'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Anna invite members to Org1?
ss.object = 'Anna'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User John edit Organization Org1?
ss.object = 'John'
action = 'edit'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Bob edit Organization Org1?
ss.object = 'Bob'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Anna edit Organization JohnOrg?
object = 'JohnOrg'
ss.object = 'Anna'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User John edit Organization JohnOrg?
ss.object = 'John'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Alice edit Organization JohnOrg?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False 

## Can User Alice delete Organization JohnOrg?
action = 'delete'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User John delete Organization JohnOrg?
ss.object = 'John'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Alice delete Organization JohnOrg?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Bob delete Organization Org2?
ss.object = 'Bob'
object = 'Org2'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == False

## Can User Alice delete Organization Org2?
ss.object = 'Alice'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True


## Testing relations
print('------------------------------')
print("[.] Testing Project -> Org Relations:")
print('------------------------------')

ss = SubjectSet(
    namespace='Organization',
    object='Org1',
)
relation = 'parents'
namespace = "Project"
object = "Project1"

## Is Organization Org1 the parent of Project Project1?
x = check_relation_tuple(namespace, object, relation, subject_set=ss)
assert x[1] == True

## Is Organization Org1 the parent of Project JohnProject?
object = 'JohnProject'
x = check_relation_tuple(namespace, object, relation, subject_set=ss)
assert x[1] == False

## Is Organization JohnOrg the parent of Project JohnProject?
ss.object = 'JohnOrg'
x = check_relation_tuple(namespace, object, relation, subject_set=ss)
assert x[1] == True

## Is Organization JohnOrg the parent of Project Project1?
ss.object = 'JohnOrg'
object = 'Project1'
x = check_relation_tuple(namespace, object, relation, subject_set=ss)
assert x[1] == False

## Alice makes Bob an owner of Org1
print('------------------------------')
print("[.] Changing permissions User -> Org relation.")
print("[.] Bob from Viewer to Owner of Org1")
print('------------------------------')

ss = SubjectSet(
    namespace='User',
    object='Bob',
    relation='session',
)
relation = 'owners'
namespace = "Organization"
object = 'Org1'
create_relation_tuple(namespace, object, relation, subject_set=ss)

## Can User Bob edit Org Org1?
action = 'edit'
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True

## Can User Bob delete Project Project1?
action = 'delete'
namespace = "Project"
object = "Project1"
x = check_relation_tuple(namespace, object, action, subject_set=ss)
assert x[1] == True
