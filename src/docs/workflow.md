# Smart Attendance System - Workflow

## 1️⃣ Landing Page

User is presented with two options:

- **[ Login ]** → For existing users
- **[ Register ]** → For new users

---

## 2️⃣ Login Flow

1. User enters:
   - **Email** (personal email, not organization email)
   - **Password**
   - _(Optional) Organization Email_ (if user belongs to multiple organizations)
2. System verifies credentials:
   - If valid, check user role:
     - **Admin** → Redirect to **Admin Dashboard**
     - **Staff** → Redirect to **Attendance Dashboard**
     - **User** → Redirect to **User Dashboard**
3. If the email is linked to multiple organizations, prompt user to select one.
4. Generate **JWT token/session** and store for authentication.

---

## 3️⃣ Register Flow

User chooses between:

- **[ Create Organization ]** → To start a new organization
- **[ Join Organization ]** → To join an existing organization

---

## 4️⃣ Create Organization Flow

1. User enters:
   - **Admin Name**
   - **Admin Email**
   - **Password**
   - **Organization Name**
   - **Organization Email**
   - **Organization Password** (for security)
   - **Subscription Plan** (Free / Pro / Enterprise)
2. If Free Plan is chosen, check if the user already owns an organization:
   - If Yes → Show error: _"You already own an organization. Please log in."_
   - If No → Proceed
3. System creates organization and assigns user as **Admin**.
4. Redirect to **Admin Dashboard**.

---

## 5️⃣ Join Organization Flow

1. User enters:
   - **Full Name**
   - **Personal Email**
   - **Organization Email**
   - **Organization Password** (for security)
   - **Password**
2. System verifies:
   - Does the organization exist?
   - Is the organization password correct?
   - Are user credentials valid?
   - (Optional) Does an admin need to approve the request?
3. If valid, assign appropriate role:
   - **Staff** → Can mark attendance, view reports, and limited settings.
   - **User** → Can only view their own attendance.
4. Redirect to **Dashboard**.

---

## 6️⃣ Post-Login Role-Based Access

### **Admin Role**

- Can add/manage users and staff
- Can view and edit attendance records
- Can manage organization settings

### **Staff Role**

- Can mark attendance
- Can view attendance reports
- Cannot manage users or organization settings

### **User Role**

- Can only view their own attendance records

---

## 7️⃣ Logout

- Users can log out at any time, returning to the **Landing Page**.

---
