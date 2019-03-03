# Thread

## `GET /v1/thread`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an array containing all threads present on any thread.

## `GET /v1/thread/<id>`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns a `Thread` object given its id.
If the id is not known,
a `404 NOT FOUND` status will be returned.

## `GET /v1/thread/<id>/full`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns a `Thread` object given its id,
along with the sections, events, section locks, and author.
If the id is not known,
a `404 NOT FOUND` status will be returned.

## `POST /v1/thread`

This endpoint should return the HTTP status `201 CREATED`.

This endpoint returns an object containing the id of the row inserted into the database.
Additional fields should be considered an implementation detail.

## `PATCH /v1/thread/<id>`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an object containing, at a minimum,
all fields that were updated.
Additional fields should be considered an implementation detail.

## `PATCH /v1/thread/<id>/approve`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns no body object on success.
It approves the thread on Reddit
and does not alter the state in Enceladus.

## `PATCH /v1/thread/<id>/sticky` and `PATCH /v1/thread/<id>/unsticky`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns no body object on success.
It stickies or unstickies the thread on Reddit
and does not alter the state in Enceladus.

## `DELETE /v1/thread/<id>`

This endpoint should return the HTTP status `204 NO CONTENT`.

This endpoint does not return any data.
