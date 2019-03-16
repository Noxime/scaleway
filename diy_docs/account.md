# `https://account.scaleway.com/`

### `date` strings
Dates returned by the scaleway api are in ISO-6401 format, aka
`yyyy-mm-ddThh:mm:ss.<fraction>Z` ('Z' for UTC time, otherwise `+hh:mm`)
for example:
`2018-05-30T19:08:42.624871+00:00`

### Object `token`
`token` objects contain the following fields
```json
{
    "access_key": "SCWxxxxxxxxxxxxxxxxx", // string
    "category": "session", // string; one of ["session", "user_created"]
    "user_id": "<UserId>", // string; user id string
    "description": "", // string; user set description for this token
    "roles": { ... } , // `roles` object, see its documentation
    "inherits_user_perms": true, // bool
    "deletion_date": null, // Optional; `date` string; see `date` documentation
    "creation_ip": "127.0.0.1", // string; either ipv4 or ipv6
    "expires": null, // Optional; `date` string
    "creation_date": "2018-05-30T19:08:42.624871+00:00", // string: `date` string
}
```

### Object `roles`
`roles` contains the following fields
```json
{
    "organization": null, // Optional; organization id string or null
    "role": null, // Optional; role id string or null
}
```

## GET `tokens/`
Returns a list of tokens
```json
{
    "tokens": [
        {
            <token>
        },
        {
            <token>
        },
        ...
    ]
}
```