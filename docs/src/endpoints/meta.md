# Meta endpoint

The `/meta` endpoint exists to assist any automation tools you may have.
This endpoint is _not_ (and will not be) versioned,
though I do not expect any fields to be removed.
Additional fields may be added at any time without notice.

Currently, this endpoint returns the following fields:

- `version`

  The full version of the current iteration.
  This includes minor and patch releases.

- `version_major`

  The major version of the current iteration.
  You may want to periodically check this value
  to see if you need to update your code for a newer version.

- `repository`

  The URL of the API repository.
