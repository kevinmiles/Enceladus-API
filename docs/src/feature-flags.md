# Feature flags

On some endpoints,
there can be a significant number of fields.
In an effort to minimize payloads,
some of these fields will not be sent by default,
as they won't be used in most cases.

But what if you want to get those fields?
In that case, you'll have to send a "feature flag" along with your request.
All that amounts to is adding the `features` query parameter to the request URL,
with the value being all of the "features" you'd like to receive, comma separated.

How do we know what feature we need?
It's actually pretty easy!
From the type definitions,
you'll notice that some fields have two consecutive underscores in their name,
such as `space__t0`.
The feature's name is whatever comes before those underscores,
in this case `space`.
So if we wanted to receive the `space__t0` field in the response body,
we'd have to add `?features=space` to the end of your request URL.

This filtering and flag capability is present on all endpoints,
and cannot be disabled (nor is there an "all" flag).
As you should know what fields you are using before making the request,
it shouldn't be much work to determine what features you'll set with the request.
It is important to note, however,
that these flags only affect the _response_.
Any request setting the aforementioned `space__t0` field does _not_ need to add the `space` feature,
unless, of course, they would like to receive that field back in the response.
Likewise, the presence or lack of a feature on a `POST` request does not affect what fields are inserted;
all fields are always present,
even if not sent back in the response body.
