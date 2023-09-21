# decorum
forum software written in rust because i disliked all the other alternatives

## api

setup api:

`$ sqlite3 decorum.db < prepare.sql`

`$ DATABASE_URL=sqlite://decorum.db SESSION_HANDLER_TOKEN=... cargo r`
