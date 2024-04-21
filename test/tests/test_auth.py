import requests

from lib.api_config import DOMAIN
from lib.utils import random_string
from openapi_gen.openapi_client.api.users_api import UsersApi
from openapi_gen.openapi_client.api_client import ApiClient
from openapi_gen.openapi_client.configuration import Configuration


def test_auth_normal():
    email = f"{random_string(10)}@example.com"
    password = "password"

    users_api = UsersApi(ApiClient(Configuration(host=f"http://{DOMAIN}")))
    users_api.post_users(user_credentials={"email": email, "password": password})

    auth_response = requests.post(
        f"http://{DOMAIN}/auth",
        json={
            "email": email,
            "password": password,
        },
    )
    assert auth_response.status_code == 200
    assert isinstance(auth_response.json()["token"], str)


def test_auth_with_invalid_json_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/auth",
        json={
            "invalid": "invalid",
            "password": "password",
        },
    )
    assert response.status_code == 422


def test_auth_with_invalid_email_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/auth",
        json={
            "email": 0,
            "password": "password",
        },
    )
    assert response.status_code == 422


def test_auth_with_invalid_password_then_unprocessable_entity():
    response = requests.post(
        f"http://{DOMAIN}/auth",
        json={
            "email": f"{random_string(10)}@example.com",
            "password": True,
        },
    )
    assert response.status_code == 422


def test_auth_with_email_not_found_then_unauthorized():
    response = requests.post(
        f"http://{DOMAIN}/auth",
        json={
            "email": f"{random_string(10)}@example.com",
            "password": "password",
        },
    )
    assert response.status_code == 400


def test_auth_with_invalid_password_then_unauthorized():
    email = f"{random_string(10)}@example.com"
    password = "password"

    users_api = UsersApi(ApiClient(Configuration(host=f"http://{DOMAIN}")))
    users_api.post_users(user_credentials={"email": email, "password": password})

    response = requests.post(
        f"http://{DOMAIN}/auth",
        json={
            "email": email,
            "password": "invalid",
        },
    )
    assert response.status_code == 400
