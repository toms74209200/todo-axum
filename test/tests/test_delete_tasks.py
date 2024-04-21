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


def test_delete_tasks_normal():
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

    delete_response = requests.delete(
        f"http://{DOMAIN}/tasks/{task_response.id}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert delete_response.status_code == 204

    get_response = tasks_api.get_tasks(
        authorization=f"Bearer {token}",
        user_id=user_response.id,
    )
    assert get_response == []


def test_delete_tasks_with_invalid_request_then_bad_request():
    delete_response = requests.delete(f"http://{DOMAIN}/tasks/1")
    assert delete_response.status_code == 400


def test_delete_tasks_with_invalid_token_then_unauthorized():
    delete_response = requests.delete(
        f"http://{DOMAIN}/tasks/1",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
    )
    assert delete_response.status_code == 401


def test_delete_tasks_with_invalid_id_then_bad_request():
    email = f"{random_string(10)}@example.com"
    password = "password"

    users_api = UsersApi(ApiClient(Configuration(host=f"http://{DOMAIN}")))
    users_api.post_users(user_credentials={"email": email, "password": password})
    auth_response = AuthApi(api_client).post_auth(
        user_credentials={"email": email, "password": password}
    )
    token = auth_response.token

    delete_response = requests.delete(
        f"http://{DOMAIN}/tasks/1",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert delete_response.status_code == 404
