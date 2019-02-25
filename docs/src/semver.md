# Semantic versioning

This API follows [semantic versioning](https://semver.org/).
As such, all breaking changes will bump the major version.

To ensure stability among all implementations using the API,
the most recent release of the prior major version
will be fully supported for at least one month,
including updates, bug fixes, and any other open issues.
There will be at least two months from the breaking change
until the prior version's functionality is removed.

In some instances,
certain behavior may be explicitly declared _not guaranteed_.
Users of the API should ensure the existence and/or validity of the behavior in question.
Changes to this behavior will bump the minor version,
but _not_ the major,
due to the fact that it is an implementation detail.
These changes will receive at least one week notice.

These time frames may be lowered or removed
if the cause of the breaking change is the result of a security issue
or other urgent matter.
