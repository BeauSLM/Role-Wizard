# Perms/Intents

This (for now) needs:
- the "Message Content" intent enabled
- Send messages perm
- Manage roles perm

The perms int is 268437504

# Setup

Find a way to set the DISCORD_TOKEN env var to your bot token. I'm using direnv.
If you want, you could modify the source to use dotenv, doesn't really matter.

For direnv:
```
# .envrc

export DISCORD_TOKEN=<your token here without angle brackers>
```

Then run `direnv allow`
