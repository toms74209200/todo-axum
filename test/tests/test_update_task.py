import datetime

import requests

from lib.api_config import DOMAIN
from lib.utils import random_string
from openapi_gen.openapi_client.api.auth_api import AuthApi
from openapi_gen.openapi_client.api.tasks_api import TasksApi
from openapi_gen.openapi_client.api.users_api import UsersApi
from openapi_gen.openapi_client.api_client import ApiClient
from openapi_gen.openapi_client.configuration import Configuration

api_client = ApiClient(Configuration(host=f"http://{DOMAIN}"))


def test_update_task_normal():
    email = f"{random_string(10)}@example.com"
    password = "password"

    user_response = UsersApi(api_client).post_users(
        user_credentials={"email": email, "password": password}
    )
    auth_response = AuthApi(api_client).post_auth(
        user_credentials={"email": email, "password": password}
    )
    token = auth_response.token

    deadline = datetime.datetime.now() + datetime.timedelta(days=1)
    tasks_api = TasksApi(api_client)
    task_response = tasks_api.post_tasks(
        authorization=f"Bearer {token}",
        post_tasks_request={
            "name": "task1",
            "description": "description1",
            "deadline": deadline.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )
    tasks_api.post_tasks(
        authorization=f"Bearer {token}",
        post_tasks_request={
            "name": "task2",
            "description": "description2",
            "deadline": datetime.datetime.now().strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )

    name = "updated_task1"
    description = "updated"
    deadline = datetime.datetime.now() + datetime.timedelta(days=2)
    completed = True
    update_response = requests.put(
        f"http://{DOMAIN}/tasks/{task_response.id}",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": name,
            "description": description,
            "deadline": deadline.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": completed,
        },
    )

    assert update_response.status_code == 200
    assert update_response.json()["name"] == name
    assert update_response.json()["description"] == description
    assert update_response.json()["deadline"] == deadline.strftime("%Y-%m-%dT%H:%M:%SZ")
    assert update_response.json()["completed"] == completed

    get_response = requests.get(
        f"http://{DOMAIN}/tasks?userId={user_response.id}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )

    assert get_response.status_code == 200
    assert get_response.json()[0]["name"] == name
    assert get_response.json()[0]["description"] == description
    assert get_response.json()[0]["deadline"] == deadline.strftime("%Y-%m-%dT%H:%M:%SZ")
    assert get_response.json()[0]["completed"] == completed


def test_update_tasks_with_invalid_request_then_bad_request():
    update_response = requests.put(f"http://{DOMAIN}/tasks/1", json={})
    assert update_response.status_code == 400


def test_update_tasks_with_invalid_token_then_unauthorized():
    update_response = requests.put(
        f"http://{DOMAIN}/tasks/1",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
        json={},
    )
    assert update_response.status_code == 401


def test_update_tasks_with_invalid_id_then_not_found():
    email = f"{random_string(10)}@example.com"
    password = "password"

    UsersApi(api_client).post_users(
        user_credentials={"email": email, "password": password}
    )
    auth_response = AuthApi(api_client).post_auth(
        user_credentials={"email": email, "password": password}
    )
    token = auth_response.token

    update_response = requests.put(
        f"http://{DOMAIN}/tasks/1",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": "task1",
            "description": "description1",
            "deadline": datetime.datetime.now().strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )
    assert update_response.status_code == 404
