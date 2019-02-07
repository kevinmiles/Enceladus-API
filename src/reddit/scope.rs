use std::fmt::{self, Display, Formatter};

pub enum Scope {
    /// Update preferences and related account information.
    /// Will not have access to your email or password.
    Account,

    /// Spend my reddit gold creddits on giving gold to other users.
    Creddits,

    /// Edit and delete my comments and submissions.
    Edit,

    /// Select my subreddit flair.
    /// Change link flair on my submissions.
    Flair,

    /// Access my voting history and comments or submissions I've saved or hidden.
    History,

    /// Access my reddit username and signup date.
    Identity,

    /// Manage settings and contributors of live threads I contribute to.
    LiveManage,

    /// Manage the configuration, sidebar, and CSS of subreddits I moderate.
    ModConfig,

    /// Add/remove users to approved submitter lists and
    /// ban/unban or mute/unmute users from subreddits I moderate.
    ModContributors,

    /// Manage and assign flair in subreddits I moderate.
    ModFlair,

    /// Access the moderation log in subreddits I moderate.
    ModLog,

    /// Access and manage modmail via mod.reddit.com.
    ModMail,

    /// Invite or remove other moderators from subreddits I moderate.
    ModOthers,

    /// Approve, remove, mark nsfw, and distinguish content in subreddits I moderate.
    ModPosts,

    /// Accept invitations to moderate a subreddit.
    /// Remove myself as a moderator or contributor of subreddits I moderate or contribute to.
    ModSelf,

    /// Access traffic stats in subreddits I moderate.
    ModTraffic,

    /// Change editors and visibility of wiki pages in subreddits I moderate.
    ModWiki,

    /// Access the list of subreddits I moderate, contribute to, and subscribe to.
    MySubreddits,

    /// Access my inbox and send private messages to other users.
    PrivateMessages,

    /// Access posts and comments through my account.
    Read,

    /// Report content for rules violations. Hide & show individual submissions.
    Report,

    /// Save and unsave comments and submissions.
    Save,

    /// Edit structured styles for a subreddit I moderate.
    StructuredStyles,

    /// Submit links and comments from my account.
    Submit,

    /// Manage my subreddit subscriptions. Manage "friends" — users whose content I follow.
    Subscribe,

    /// Submit and change my votes on comments and submissions.
    Vote,

    /// Edit wiki pages on my behalf
    WikiEdit,

    /// Read wiki pages through my account
    WikiRead,
}

impl Display for Scope {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Scope::*;
        let value = match self {
            Account => "account",
            Creddits => "creddits",
            Edit => "edit",
            Flair => "flair",
            History => "history",
            Identity => "identity",
            LiveManage => "livemanage",
            ModConfig => "modconfig",
            ModContributors => "modcontributors",
            ModFlair => "modflair",
            ModLog => "modlog",
            ModMail => "modmail",
            ModOthers => "modothers",
            ModPosts => "modposts",
            ModSelf => "modself",
            ModTraffic => "modtraffic",
            ModWiki => "modwiki",
            MySubreddits => "mysubreddits",
            PrivateMessages => "privatemessages",
            Read => "read",
            Report => "report",
            Save => "save",
            StructuredStyles => "structuredstyles",
            Submit => "submit",
            Subscribe => "subscribe",
            Vote => "vote",
            WikiEdit => "wikiedit",
            WikiRead => "wikiread",
        };

        write!(f, "{}", value)
    }
}
