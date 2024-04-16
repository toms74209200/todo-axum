import datetime
import requests

from lib.utils import random_string


def test_post_task_normal():
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
    token = auth_response.json()["token"]

    deadline = datetime.datetime.now() + datetime.timedelta(days=1)
    task_response = requests.post(
        "http://localhost:3000/tasks",
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
        "http://localhost:3000/tasks",
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
        "http://localhost:3000/tasks",
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
        "http://localhost:3000/tasks",
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
        "http://localhost:3000/tasks",
        json={
            "name": "task1",
            "description": "description1",
            "deadline": 0,
            "completed": False,
        },
    )
    assert response.status_code == 422


def test_post_task_with_invalid_token_then_bad_request():
    response = requests.post(
        "http://localhost:3000/tasks",
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
