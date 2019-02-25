# User

## `GET /v1/user`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an array containing all users present on any user.

## `GET /v1/user/<id>`

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an `User` object given their id.
If the id is not known,
a `404 NOT FOUND` status will be returned.

## `POST /v1/user`

**This endpoint is only present during testing.**
The creation of users should never be manual;
it should only occur via the OAuth flow.

This endpoint should return the HTTP status `201 CREATED`.

This endpoint returns an object containing the id of the row inserted into the database.
Additional fields should be considered an implementation detail.

## `PATCH /v1/user/<id>`

**This endpoint is only present during testing.**
The modification of users should never be manual;
it should only be done via internal methods.

This endpoint should return the HTTP status `200 OK`.

This endpoint returns an object containing, at a minimum,
all fields that were updated.
Additional fields should be considered an implementation detail.

## `DELETE /v1/user/<id>`

**This endpoint is only present during testing.**
The deletion of users should never be manual;
it should only be done via internal methods.

This endpoint should return the HTTP status `204 NO CONTENT`.

This endpoint does not return any data.
