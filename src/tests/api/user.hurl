# Test Login API
POST http://localhost:8000/login
Content-Type: application/json

{
    "identifier": "h1@gmail.com",
    "password": "securepassword"
}

# Test Logout API
DELETE http://localhost:8000/logout

# Create a new user
POST http://localhost:8000/users/new
Content-Type: application/json

{
  "name": "user1",
  "email": "h1@gmail.com",
  "password": "securepassword",
  "subscription_plan": "Free"
}

# Get all users
GET http://localhost:8000/users/all

# Get user
GET http://localhost:8000/users/h1@gmail.com

# Delete user 
DELETE http://localhost:8000/users/h1@gmail.com

# Update user
PUT http://localhost:8000/users/h1@gmail.com
Content-Type: application/json

{
  "name": "updatedUser1",
  "email": "h1@gmail.com",
  "subscription_plan": "Pro"
}
