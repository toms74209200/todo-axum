import datetime
import requests

from lib.utils import random_string


def test_delete_tasks_normal():
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

    delete_response = requests.delete(
        f"http://localhost:3000/tasks/{task_response.json()['id']}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert delete_response.status_code == 204

    get_response = requests.get(
        f"http://localhost:3000/tasks?userId={user_response.json()['id']}",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert get_response.json() == []


def test_delete_tasks_with_invalid_request_then_bad_request():
    delete_response = requests.delete("http://localhost:3000/tasks/1")
    assert delete_response.status_code == 400


def test_delete_tasks_with_invalid_token_then_unauthorized():
    delete_response = requests.delete(
        "http://localhost:3000/tasks/1",
        headers={
            "Authorization": "Bearer " + random_string(100),
        },
    )
    assert delete_response.status_code == 401


def test_delete_tasks_with_invalid_id_then_bad_request():
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

    delete_response = requests.delete(
        f"http://localhost:3000/tasks/1",
        headers={
            "Authorization": f"Bearer {token}",
        },
    )
    assert delete_response.status_code == 404
