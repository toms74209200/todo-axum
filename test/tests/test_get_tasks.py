import datetime
import requests

from lib.utils import random_string


def test_get_tasks_normal():
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

    get_response = requests.get(
        f"http://localhost:3000/tasks?userId={user_response.json()['id']}",
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
    response = requests.get("http://localhost:3000/tasks")
    assert response.status_code == 400


def test_get_tasks_with_invalid_token_then_unauthorized():
    response = requests.get(
        "http://localhost:3000/tasks?userId=0",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
    )
    assert response.status_code == 401


def test_get_tasks_with_user_id_not_found_then_bad_request():
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

    get_response = requests.get(
        "http://localhost:3000/tasks?userId=10000",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert get_response.status_code == 400
