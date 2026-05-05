# API Contract — Week Tech

## POST /registrations

```json
{
  "name": "string", // 255 characters max
  "student_registration": 253579732, // 9 chars max
  "course_name": "string", // 255 characters max
  "course_period": 1, // min 1, max 12
  "coffee_break": false/true
}
```

- `200`
- `400` `{ "error": "missing_fields" }`
- `400` `{ "error": "invalid_ra" }`
- `400` `{ "error": "ra_already_registered" }`
- `422` `{ "error": "student_registration: expected number" }`

---

## POST /projects

```json
{
  "submitter_name": "string", // 255 characters max
  "submitter_registration": 253579732, // 9 chars max
  "project_name": "string", // 255 characters max
  "description": "string" // 500 characters max
}
```

- `200`
- `400` `{ "error": "missing_fields" }`
- `400` `{ "error": "invalid_ra" }`

---

## POST /checkin

```json
{
  "student_registration": 253579732 // - 9 chars max
}
```

- `200`
- `400` `{ "error": "invalid_ra" }`
- `404` `{ "error": "ra_not_found" }`

---

## POST /admin/login

```json
{
  "email": "string", // email type
  "password": "string" // backend validation
}
```

- `200` `{ "token": "..."}`
- `400` `{ "error": "missing_fields" }`
- `401` `{ "error": "invalid_credentials" }`

---

## GET /registrations — protected (JWT)

```json
[
  {
    "name": "string", // 255 characters max
    "student_registration": 253579732, // 9 chars max
    "course_name": "string", // 255 characters max
    "course_period": 1, // min 1, max 12
    "coffee_break": false,
    "checked_in": false
  }
]
```

---

## GET /projects — protected (JWT)

```json
[
  {
    "id": 1,
    "submitter_name": "string", // 255 characters max
    "submitter_registration": 253579732, // 9 chars max
    "project_name": "string", // 255 characters max
    "description": "string" // 500 characters max
  }
]
```
