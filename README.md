<img align="left" width="200" src="https://raw.githubusercontent.com/rust-lang-ve/design/main/assets/logo_above.png" />

# Hilow Social Feed API

"Hilow" is a microblogging social feed where users are able to share they thoughts on different topics

<hr />


## Contents

- [Requirements](#requirements)
- [API](#api)
  - [Session Management](#session-management)
  - [User Management](#user-management)
  - [Post Management](#post-management)
  - [Feed Management](#feed-management)
- [Contracts](#contracts)
  - [LoginRequest](#loginrequest)
  - [RegisterRequest](#registerrequest)
- [Entities](#entities)
  - [User](#user)
  - [Secret](#secret)
  - [Profile](#profile)
  - [File](#file)
  - [Post](#post)
  - [Hashtag](#hashtag)

## Requirements

- Docker
- Rust

## API

**Root Path**: api/v1/

### Session Management

URI | Method | Req. Body | Res. Body | Description
--- | --- | --- | --- | ---
`login` | POST | [Login Req](#LoginRequest) | [Login Resp](#LoginResponse) | Update current logged User Info.
`logout` | POST | - | - | Update current logged User Info.
`register` | POST | [Register Request](#RegisterRequest) | [Register Response](#RegisterResponse) | Register a new User.

### User Management

URI | Method | Req. Body | Res. Body | Description
--- | --- | --- | --- | ---
`users/me` | GET | - | [User](#user) | Get current Logged User Info.
`users` | PUT | - | [User](#user) | Update current logged User Info.
`users/:username` | GET | - | [User](#user) | Get  User Info by User name.
`users/:username/posts` | GET | - | Array<[Post](#post)> | Get  User Info by User name.
`users/:username/posts/:id` | GET | - | [User](#user) | Get  User Info by User name.
`users/:username/profile` | PUT | - | [User](#user) | Update User Profile.


### Post Management

URI | Method | Req. Body | Res. Body | Description
--- | --- | --- | --- | ---
`posts` | POST | - | [Post](#post) | Create a Post by current logged User.
`posts/:postId` | PATCH | - | [Post](#post) | Update a Post only if belongs to Logged User.
`posts/:postId` | DELETE | - | - | Delete a Post only if belongs to Logged User.

### Feed Management

URI | Method | Req. Body | Res. Body | Description
--- | --- | --- | --- | ---
`feed` | Get | - | Array<[Post](#post)> | Get feed based on current logged user.

## Contracts

### LoginRequest

Property | Type | Optional | Description
--- | --- | --- | ---
`user_id` | `string` | No | User id (Email or Username).
`password` | `string` | No | User's Password.


### RegisterRequest

Property | Type | Optional | Description
--- | --- | --- | ---
`email` | `string` | No | User's Email.
`username` | `string` | No | User's Username.
`name` | `string` | No | User's name.
`surname` | `string` | yes | User's surname.
`password` | `string` | No | User's Password.

## Entities

### User

A platform user with unique identification (email).

Property | Type | Optional | Description
--- | --- | --- | ---
`id` | `UUID` | No | Unique identifier for registry.
`email` | `string` | No | User's Email.
`username` | `string` | No | User's Username.

### Secret

A `User`'s secret may contain the password hash used to validate future authentication process

Property | Type | Optional | Description
--- | --- | --- | ---
`id` | `UUID` | No | Unique identifier for registry.
`user_id` | `UUID` | No | Foreign key for `User` relationship.
`hash` | `string` | No | Password hash stored when signing up or updating password details.

### Profile

A `User`'s profile contains details on the person behind an `User` entry

Property | Type | Optional | Description
--- | --- | --- | ---
`id` | `UUID` | No | Unique identifier for registry.
`user_id` | `UUID` | No | Foreign key for `User` relationship.
`avatar_id` | `UUID` | Yes | Foreign key for `File` relationship used as Avatar Image.
`name` | `string` | No | User's profile name.
`surname` | `string` | Yes | User's profile surname.

### File

Platform file entry used for posting image files and retrieving them as well

Property | Type | Optional | Description
--- | --- | --- | ---
`id` | `UUID` | No | Unique identifier for registry.
`mime_type` | `string` | No | File's MIME type. [Refer to MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types)
`bytes` | `Array<u8>` | No | Raw bytes from the file.
`size` | `uint` | No | Size of the file in question.
`url` | `string` | No | File's public access URL.
`sha256` | `string` | No | File's SHA256 checksum.

### Post

A feed `Post` created by any user in the platform.
`Post`s may or may not have parent nodes used to achieve comment-like functionality.

Posts may also contain one or more `File`s as well.

Property | Type | Optional | Description
--- | --- | --- | ---
`id` | `UUID` | No | Unique identifier for registry.
`user_id` | `UUID` | No | Foreign key for `User` relationship. Also refers to the owner of this file, bringing access to File deletion for instance.
`parent_id` | `UUID` | Yes | Foreign key for `Post` relationship. This is a recursive relationship and may not be present if the Post doesn't have a parent.
`files` | `Array<File>` | No | An array of `File` entries which belongs to the Post. A Post may not have `File` entries related to it, for such cases this array must remain empty instead.
`body` | `string` | Yes | Text content for posts, this contents may contain HTML. If thats the case, such HTML must be escaped.
`hashtags` | `Array<Hashtag>` | No | A post may or may not contain `Hashtag` instances related to itself. As well as the `files` array, the `hashtags` array must be present always, if theres no `Hashtag` relationships present then this array must remain empty.

### Hashtag

A `Post`'s hashtag used to "tag" posts with a unique name/key

Property | Type | Optional | Description
--- | --- | --- | ---
`id` | `UUID` | No | Unique identifier for registry
`name` | `string` | No | Name for the Hashtag

## License

Licensed under the GNU License

## Contributions

Contributions of any kind are welcome.
Feel free to either open a Pull Request or file an issue to participate in this project.
