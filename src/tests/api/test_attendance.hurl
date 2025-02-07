# Create a new Attendance
POST http://localhost:8000/attendances/new
Content-Type: application/json

{
    "student_id": "60c72b2f9e1b8f7f5f8a5c64",
    "class_id": "60c72b2f9e1b8f7f5f8a5c64",
    "teacher_id": "60c72b2f9e1b8f7f5f8a5c64",
    "date": "2025-02-06",
    "status": "Present"
}

HTTP/1.1 201

# Get all Attendance records
GET http://localhost:8000/attendances/all
HTTP/1.1 200

# Get a specific Attendance record by ID (replace {attendance_id} with an actual attendance ID)
GET http://localhost:8000/attendances/60c72b2f9e1b8f7f5f8a5c64
HTTP/1.1 200

# Update an Attendance (replace {attendance_id} with an actual attendance ID)
PUT http://localhost:8000/attendances/60c72b2f9e1b8f7f5f8a5c64
Content-Type: application/json

{
    "student_id": "60c72b2f9e1b8f7f5f8a5c64",
    "class_id": "60c72b2f9e1b8f7f5f8a5c64",
    "teacher_id": "60c72b2f9e1b8f7f5f8a5c64",
    "date": "2025-02-06",
    "status": "Absent"
}

HTTP/1.1 200

# Delete an Attendance (replace {attendance_id} with an actual attendance ID)
DELETE http://localhost:8000/attendances/60c72b2f9e1b8f7f5f8a5c64
HTTP/1.1 200

