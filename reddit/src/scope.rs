use enum_display::Display;

/// All possible scopes that can be requested from a user when authenticating.
///
/// Descriptions are from Reddit.
#[derive(Display)]
pub enum Scope {
    /// Update preferences and related account information.
    /// Will not have access to your email or password.
    #[display = "account"]
    Account,

    /// Spend my reddit gold creddits on giving gold to other users.
    #[display = "creddits"]
    Creddits,

    /// Edit and delete my comments and submissions.
    #[display = "edit"]
    Edit,

    /// Select my subreddit flair.
    /// Change link flair on my submissions.
    #[display = "flair"]
    Flair,

    /// Access my voting history and comments or submissions I've saved or hidden.
    #[display = "history"]
    History,

    /// Access my reddit username and signup date.
    #[display = "identity"]
    Identity,

    /// Manage settings and contributors of live threads I contribute to.
    #[display = "livemanage"]
    LiveManage,

    /// Manage the configuration, sidebar, and CSS of subreddits I moderate.
    #[display = "modconfig"]
    ModConfig,

    /// Add/remove users to approved submitter lists and
    /// ban/unban or mute/unmute users from subreddits I moderate.
    #[display = "modcontributors"]
    ModContributors,

    /// Manage and assign flair in subreddits I moderate.
    #[display = "modflair"]
    ModFlair,

    /// Access the moderation log in subreddits I moderate.
    #[display = "modlog"]
    ModLog,

    /// Access and manage modmail via mod.reddit.com.
    #[display = "modmail"]
    ModMail,

    /// Invite or remove other moderators from subreddits I moderate.
    #[display = "modothers"]
    ModOthers,

    /// Approve, remove, mark nsfw, and distinguish content in subreddits I moderate.
    #[display = "modposts"]
    ModPosts,

    /// Accept invitations to moderate a subreddit.
    /// Remove myself as a moderator or contributor of subreddits I moderate or contribute to.
    #[display = "modself"]
    ModSelf,

    /// Access traffic stats in subreddits I moderate.
    #[display = "modtraffic"]
    ModTraffic,

    /// Change editors and visibility of wiki pages in subreddits I moderate.
    #[display = "modwiki"]
    ModWiki,

    /// Access the list of subreddits I moderate, contribute to, and subscribe to.
    #[display = "mysubreddits"]
    MySubreddits,

    /// Access my inbox and send private messages to other users.
    #[display = "privatemessages"]
    PrivateMessages,

    /// Access posts and comments through my account.
    #[display = "read"]
    Read,

    /// Report content for rules violations. Hide & show individual submissions.
    #[display = "report"]
    Report,

    /// Save and unsave comments and submissions.
    #[display = "save"]
    Save,

    /// Edit structured styles for a subreddit I moderate.
    #[display = "structuredstyles"]
    StructuredStyles,

    /// Submit links and comments from my account.
    #[display = "submit"]
    Submit,

    /// Manage my subreddit subscriptions. Manage "friends" — users whose content I follow.
    #[display = "subscribe"]
    Subscribe,

    /// Submit and change my votes on comments and submissions.
    #[display = "vote"]
    Vote,

    /// Edit wiki pages on my behalf
    #[display = "wikiedit"]
    WikiEdit,

    /// Read wiki pages through my account
    #[display = "wikiread"]
    WikiRead,
}
