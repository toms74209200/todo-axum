import requests

from lib.utils import random_string


def test_auth_normal():
    email = f"{random_string(10)}@example.com"
    password = "password"

    user_response = requests.post(
        "http://localhost:3000/users",
        json={
            "email": email,
            "password": password,
        },
    )
    assert user_response.status_code == 201

    auth_response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": email,
            "password": password,
        },
    )
    assert auth_response.status_code == 200
    assert isinstance(auth_response.json()["token"], str)


def test_auth_with_invalid_json_then_unprocessable_entity():
    response = requests.post(
        "http://localhost:3000/auth",
        json={
            "invalid": "invalid",
            "password": "password",
        },
    )
    assert response.status_code == 422


def test_auth_with_invalid_email_then_unprocessable_entity():
    response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": 0,
            "password": "password",
        },
    )
    assert response.status_code == 422


def test_auth_with_invalid_password_then_unprocessable_entity():
    response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": f"{random_string(10)}@example.com",
            "password": True,
        },
    )
    assert response.status_code == 422


def test_auth_with_email_not_found_then_unauthorized():
    response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": f"{random_string(10)}@example.com",
            "password": "password",
        },
    )
    assert response.status_code == 400


def test_auth_with_invalid_password_then_unauthorized():
    email = f"{random_string(10)}@example.com"
    password = "password"

    user_response = requests.post(
        "http://localhost:3000/users",
        json={
            "email": email,
            "password": password,
        },
    )
    assert user_response.status_code == 201

    response = requests.post(
        "http://localhost:3000/auth",
        json={
            "email": email,
            "password": "invalid",
        },
    )
    assert response.status_code == 400
