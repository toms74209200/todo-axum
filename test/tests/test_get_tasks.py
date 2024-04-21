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


def test_get_tasks_normal():
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
    tasks_api.post_tasks(
        authorization=f"Bearer {token}",
        post_tasks_request={
            "name": "task1",
            "description": "description1",
            "deadline": deadline.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )

    get_response = requests.get(
        f"http://{DOMAIN}/tasks?userId={user_response.id}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert get_response.status_code == 200
    assert isinstance(get_response.json(), list)
    assert len(get_response.json()) == 1
    assert get_response.json()[0]["name"] == "task1"
    assert get_response.json()[0]["description"] == "description1"
    assert get_response.json()[0]["deadline"] == deadline.strftime("%Y-%m-%dT%H:%M:%SZ")
    assert get_response.json()[0]["completed"] == False


def test_get_tasks_with_invalid_post_then_bad_request():
    response = requests.get(f"http://{DOMAIN}/tasks")
    assert response.status_code == 400


def test_get_tasks_with_invalid_token_then_unauthorized():
    response = requests.get(
        f"http://{DOMAIN}/tasks?userId=0",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
    )
    assert response.status_code == 401


def test_get_tasks_with_user_id_not_found_then_bad_request():
    email = f"{random_string(10)}@example.com"
    password = "password"

    UsersApi(api_client).post_users(
        user_credentials={"email": email, "password": password}
    )
    auth_response = AuthApi(api_client).post_auth(
        user_credentials={"email": email, "password": password}
    )
    token = auth_response.token

    deadline = datetime.datetime.now() + datetime.timedelta(days=1)
    task_response = requests.post(
        f"http://{DOMAIN}/tasks",
        headers={
            "Authorization": f"Bearer {token}",
        },
        json={
            "name": "task1",
            "description": "description1",
            "deadline": deadline.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "completed": False,
        },
    )
    assert task_response.status_code == 201

    get_response = requests.get(
        f"http://{DOMAIN}/tasks?userId=10000",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert get_response.status_code == 400
