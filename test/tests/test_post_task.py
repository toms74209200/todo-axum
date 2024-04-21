import datetime

import requests

from lib.api_config import DOMAIN
from lib.utils import random_string
from openapi_gen.openapi_client.api.auth_api import AuthApi
from openapi_gen.openapi_client.api.users_api import UsersApi
from openapi_gen.openapi_client.api_client import ApiClient
from openapi_gen.openapi_client.configuration import Configuration

api_client = ApiClient(Configuration(host=f"http://{DOMAIN}"))


def test_post_task_normal():
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
    assert isinstance(task_response.json()["id"], int)


def test_post_task_with_invalid_json_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/tasks",
        json={
            "invalid": "invalid",
            "description": "description1",
            "deadline": "2021-01-01T00:00:00Z",
            "completed": False,
        },
    )
    assert response.status_code == 422


def test_post_task_with_invalid_name_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/tasks",
        json={
            "name": 0,
            "description": "description1",
            "deadline": "2021-01-01T00:00:00Z",
            "completed": False,
        },
    )
    assert response.status_code == 422


def test_post_task_with_invalid_description_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/tasks",
        json={
            "name": "task1",
            "description": 0,
            "deadline": "2021-01-01T00:00:00Z",
            "completed": False,
        },
    )
    assert response.status_code == 422


def test_post_task_with_invalid_deadline_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/tasks",
        json={
            "name": "task1",
            "description": "description1",
            "deadline": 0,
            "completed": False,
        },
    )
    assert response.status_code == 422


def test_post_task_with_invalid_token_then_unauthorized():
    response = requests.post(
        f"http://{DOMAIN}/tasks",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
        json={
            "name": "task1",
            "description": "description1",
            "deadline": "2021-01-01T00:00:00Z",
            "completed": False,
        },
    )
    assert response.status_code == 401
