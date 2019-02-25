# Permissions

When passing a user through OAuth,
they will currently be asked certain permissions.
How you inform end users about the details below is up to you;
there's certainly no requirement to do so.

Users will currently be asked to provide the following permissions:

> Update preferences and related account information.
> Will not have access to your email or password.

The API _does not_ update preferences.
It will only ever _read_ the preferences;
currently that is limited to the user's preferred language.

> Edit and delete my comments and submissions.

This one's pretty self-explanatory.
We're going to have to be able to
update (and possible delete) the threads that are created.

> Submit links and comments from my account.

**This description is inaccurate.**
The API will _also_ be able to submit self-posts from the account;
I have previously contacted the Reddit admins regarding this,
and received a response (albeit months later) from their legal team.
Apparently that didn't matter,
as the permissions prompt has not changed.

As to the permission itself,
threads must be able to be posted.
There doesn't _currently_ exist the ability to submit comments from Enceladus,
but that may be an option in the future.

> Approve, remove, mark nsfw, and distinguish content in subreddits I moderate.

This permission will only be used in specific subreddits,
and only when explicitly requested by the end user.
It will primarily be used to approve posts be non-approved submitters.
Without this permission, moderators would have to switch back to Reddit to do so.
This also allows moderators to sticky threads.

> Manage and assign flair in subreddits I moderate.

Similar to the previous one,
this is primarily to prevent moderators needing to switch back to Reddit.

> Access my reddit username and signup date.

I'd like to be able to know who you are!
Although there's no strict requirement on API implementations,
the main use for this will likely be to show who created a thread,
and who currently possesses section locks.

> Maintain this access indefinitely (or until manually revoked).

Without requesting permanent access,
the API would be required to re-authenticate the user every hour.
