# Test Login API
POST http://localhost:8000/login
Content-Type: application/json

{
    "identifier": "user2@example.com",
    "password": "securepassword"
}

# HTTP/1.1 200

# Test Logout API
DELETE http://localhost:8000/logout
# HTTP/1.1 200


# Create a new user
POST http://localhost:8000/users/new
Content-Type: application/json

{
  "name": "user3",
  "phone_number": "085733300379",
  "password": "securepassword"
}

# HTTP/1.1 201

# Create a new Teacher user
POST http://localhost:8000/users/new
Content-Type: application/json

{
  "username": "teacher_user",
  "email": "teacher@example.com",
  "password": "securepassword",
  "phone": "+628987654321",
  "nim": null,
  "nidn": "0987654321",
  "role": "Teacher"
}

HTTP/1.1 201

# Create a new Admin user
POST http://localhost:8000/users/new
Content-Type: application/json

{
  "username": "Rendi VS",
  "email": "admin@gmail.com",
  "password": "securepassword",
  "phone": "+628112233445",
  "role": "Admin"
}

# HTTP/1.1 201

# Get all users
GET http://localhost:8000/users/all
HTTP/1.1 200

# Get a user by ID (replace {id} with an actual user ID)
GET http://localhost:8000/users/657a8b9c1234567890abcdef
HTTP/1.1 200

# Update a user (replace {id} with an actual user ID)
PUT http://localhost:8000/users/657a8b9c1234567890abcdef
Content-Type: application/json

{
  "username": "updated_user",
  "email": "updated@example.com",
  "password": "newpassword123",
  "phone": "+628987654321",
  "nim": "1234567890",
  "nidn": "0987654321",
  "role": "Admin"
}

HTTP/1.1 200
