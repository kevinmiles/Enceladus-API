# Section

## `GET /v1/section`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an array containing all sections present on any thread.

## `GET /v1/section/<id>`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an `Section` object given their id.
If the id is not known,
a `404 NOT FOUND` status will be returned.

## `POST /v1/section`

This endpoint should return the HTTP status `201 CREATED`.

This endpoint returns an object containing the id of the row inserted into the database.
Additional fields should be considered an implementation detail.

## `PATCH /v1/section/<id>`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an object containing, at a minimum,
all fields that were updated.
Additional fields should be considered an implementation detail.

## `DELETE /v1/section/<id>`

This endpoint should return the HTTP status `204 NO CONTENT`.

This endpoint does not return any data.
