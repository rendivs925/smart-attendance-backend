# Create a new Class
POST http://localhost:8000/classes/new
Content-Type: application/json

{
    "class_name": "Programming 101",
    "teacher_id": "60c72b2f9e1b8f7f5f8a5c64"
}

HTTP/1.1 201

# Get all Classes
GET http://localhost:8000/classes/all
HTTP/1.1 200

# Get a Class by ID (replace {class_id} with an actual class ID)
GET http://localhost:8000/classes/67a4a58df92aaecb08d0e795
HTTP/1.1 200

# Update a Class (replace {class_id} with an actual class ID)
PUT http://localhost:8000/classes/67a4a58df92aaecb08d0e795
Content-Type: application/json

{
    "class_name": "Advanced Math 101",
    "teacher_id": "60c72b2f9e1b8f7f5f8a5c64"
}

HTTP/1.1 200

# Delete a Class (replace {class_id} with an actual class ID)
DELETE http://localhost:8000/classes/60c72b2f9e1b8f7f5f8a5c64
HTTP/1.1 200

