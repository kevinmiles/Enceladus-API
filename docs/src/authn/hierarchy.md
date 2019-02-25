# Hierarchy

From lowest to highest,
authentication levels are as follows.

- None

  This is exactly what it sounds like.
  No special care needs to be taken;
  any authentication headers passed are ignored.

- Signed in

  The user must have gone through the OAuth procedure.
  It does not matter _which_ user is authenticated,
  only that one is.

- Thread author

  The authenticated user must be the same user that initially created the thread.
  This is generally used when it is necessary
  to ensure a user has the authority to make changes to a given
  entity.

- Local admin

  The authenticated user is designated as an admin of the subreddit in question.
  This designation is located in the `[SUBREDDIT]__is_admin` field on each user.
  If the field does not exist,
  the value is assumed to be `false`.
  These users have the ability to add, edit, and remove sections and events
  in the same manner as the thread's author.

- Global admin

  The authenticated user is designated as a global admin.
  This designation is indicated by the `is_global_admin` field on each user.
  These users have the same abilities as local admins,
  but on all subreddits (even those not explicitly listed in the database).

For all authentication levels other than 'none',
the [JSON web token](https://jwt.io/introduction) _must_ be passed to the server for each request.
Failure to do so will result in a `401 UNAUTHORIZED` response.
The proper header is of the format `Authorization: Bearer [TOKEN]`.
