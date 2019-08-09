from datetime import datetime

from gql import gql
from common import client


def get_team_list():
    result = client.execute(gql('''
        query {
            teams {
                id
                name
            }
        }
    '''))

    return result['teams']


def get_service_list():
    result = client.execute(gql('''
        query {
            services {
                id
                name
            }
        }
    '''))

    return result['services']


# get an exploit ID with service name and filename
# create one if there doesn't exist one
def get_exploit_id(service_name, filename):
    exploit_name = '{}/{}'.format(service_name, filename)
    result = client.execute(gql('''
        query ($exploit_name: String!) {
            exploit(where: {
                name: $exploit_name
            }) {
                id
            }
        }
    '''), {"exploit_name": exploit_name})

    if result['exploit']:
        return result['exploit']['id']

    result = client.execute(gql('''
        mutation ($exploit_name: String!, $service_name: String!) {
            createExploit(data: {
                name: $exploit_name
                target: {
                    connect: {
                        name: $service_name
                    }
                }
            }) {
                id
            }
        }
    '''), {"exploit_name": exploit_name, "service_name": service_name})

    return result['createExploit']['id']


# get an endpoint ID with service name and team name
def get_endpoint_id(service_name, team_name):
    result = client.execute(gql('''
        query ($service_name: String!, $team_name: String!) {
            endpoints (where: {
                AND: [
                    {
                        team: {
                            name: $team_name
                        },
                    }
                    {
                        service: {
                            name: $service_name
                        }
                    }
                ]
            }) {
                id
            }
        }
    '''), {"service_name": service_name, "team_name": team_name})

    if len(result['endpoints']) > 0:
        return result['endpoints'][0]['id']

    return None


class TaskStatus(object):
    PENDING = "PENDING"
    EXPLOITING = "EXPLOITING"
    SUBMITTING = "SUBMITTING"
    SUBMIT_CORRECT = "SUBMIT_CORRECT"
    SUBMIT_DUPLICATE = "SUBMIT_DUPLICATE"
    SUBMIT_WRONG = "SUBMIT_WRONG"
    SUBMIT_SKIPPED = "SUBMIT_SKIPPED"
    EXPLOIT_INCOMPATIBLE = "EXPLOIT_INCOMPATIBLE"
    EXPLOIT_TIMEOUT = "EXPLOIT_TIMEOUT"
    EXPLOIT_ERROR = "EXPLOIT_ERROR"
    SUBMIT_ERROR = "SUBMIT_ERROR"
    UNKNOWN_ERROR = "UNKNOWN_ERROR"


# create a task and return its integer ID
def create_task(exploit_id, service_name, team_name):
    endpoint_id = get_endpoint_id(service_name, team_name)
    if endpoint_id is None:
        raise Exception("Endpoint for service {} team {} not found".format(service_name, team_name))

    result = client.execute(gql('''
        mutation ($exploit_id: ID!, $endpoint_id: ID!, $lastUpdate: DateTime!) {
            createTask(data: {
                exploit: {
                    connect: {
                        id: $exploit_id
                    }
                }
                endpoint: {
                    connect: {
                        id: $endpoint_id
                    }
                }
                lastUpdate: $lastUpdate
            }) {
                id
            }
        }
    '''), {"exploit_id": exploit_id, "endpoint_id": endpoint_id, "lastUpdate": datetime.now().isoformat()})

    return result['createTask']['id']


# update a task
# possible keys for update_input is as follows
# - status: TaskStatus
# - statusMesssage: String
# - flag: String
# - exploitStdout: String
# - exploitStderr: String
# - authStdout: String
# - authStderr: String
def update_task(task_id, update_input):
    update_input['lastUpdate'] = datetime.now().isoformat()
    result = client.execute(gql('''
        mutation ($task_id: Int!, $data: TaskUpdateInput!) {
            updateTask(data: $data, where: {
                id: $task_id
            }) {
                id
            }
        }
    '''), {"task_id": task_id, "data": update_input})

    assert result['updateTask']['id'] == task_id
