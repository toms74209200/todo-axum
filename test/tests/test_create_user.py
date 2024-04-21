import requests

from lib.api_config import DOMAIN
from lib.utils import random_string


def test_create_user_normal():
    json = {
        "email": f"{random_string(10)}@example.com",
        "password": "password",
    }
    response = requests.post(f"http://{DOMAIN}/users", json=json)
    assert response.status_code == 201
    assert isinstance(response.json()["id"], int)


def test_create_user_with_invalid_json_then_unprocessable_entity():
    json = {
        "invalid": "invalid",
        "password": "password",
    }
    response = requests.post(f"http://{DOMAIN}/users", json=json)
    assert response.status_code == 422


def test_create_user_with_invalid_email_then_unprocessable_entity():
    json = {
        "email": 0,
        "password": "password",
    }
    response = requests.post(f"http://{DOMAIN}/users", json=json)
    assert response.status_code == 422


def test_create_user_with_invalid_password_then_unprocessable_entity():
    json = {
        "email": f"{random_string(10)}@example.com",
        "password": True,
    }
    response = requests.post(f"http://{DOMAIN}/users", json=json)
    assert response.status_code == 422


def test_create_user_with_same_email_then_bad_request():
    email = f"{random_string(10)}@example.com"
    json = {
        "email": email,
        "password": "password1",
    }
    response = requests.post(f"http://{DOMAIN}/users", json=json)
    assert response.status_code == 201
    assert isinstance(response.json()["id"], int)

    response = requests.post(
        f"http://{DOMAIN}/users",
        json={
            "email": email,
            "password": "password2",
        },
    )
    assert response.status_code == 400


def test_create_user_with_invalid_email_then_bad_request():
    json = {
        "email": "invalid",
        "password": "password",
    }
    response = requests.post(f"http://{DOMAIN}/users", json=json)
    assert response.status_code == 400
