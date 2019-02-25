# Authentication flow

To authenticate a user,
you should first make a `GET` request to `/oauth`,
providing the mandatory query parameter `callback`.
Upon successful authentication,
the user will be redirected to `callback`
with the query parameter `token`;
this is the [JSON web token](https://jwt.io/introduction) that must be supplied
when requesting an authenticated endpoint.
Other query parameters present currently include `user_id`, `username`, and `lang`.

_Please note:
There is no `state` variable present,
as there is in a standard OAuth flow.
This may be added in the future._
